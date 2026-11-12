import sys
import struct
import os
import glob

def patch_file(exe_path):
    if not os.path.exists(exe_path):
        return False
    try:
        with open(exe_path, "r+b") as f:
            data = bytearray(f.read())
            
            pe_offset = struct.unpack_from("<I", data, 0x3C)[0]
            num_sections = struct.unpack_from("<H", data, pe_offset + 6)[0]
            opt_hdr_size = struct.unpack_from("<H", data, pe_offset + 0x14)[0]
            sec_table_offset = pe_offset + 0x18 + opt_hdr_size

            def rva_to_offset(rva):
                for i in range(num_sections):
                    sec = sec_table_offset + i * 40
                    vsize, vrva, rsize, roff = struct.unpack_from("<IIII", data, sec + 8)
                    if vrva <= rva < vrva + vsize:
                        return roff + (rva - vrva)
                return None

            import_rva, import_size = struct.unpack_from("<II", data, pe_offset + 0x18 + 0x70 + 8)
            import_off = rva_to_offset(import_rva)
            if not import_off:
                return False

            # Look at the first 33 original descriptors
            k32_idx = None
            path_idx = None
            for idx in range(33):
                desc = data[import_off + idx*20 : import_off + (idx+1)*20]
                oft, ts, fc, nrva, ft = struct.unpack("<IIIII", desc)
                if oft == 0 and nrva == 0 and ft == 0:
                    break
                noff = rva_to_offset(nrva)
                if noff:
                    name = data[noff:data.find(b"\0", noff)].decode("latin1", errors="ignore")
                    if name.lower() == "kernel32.dll" and idx > 15:
                        k32_idx = idx
                    elif "api-ms-win-core-path" in name.lower():
                        path_idx = idx

            if k32_idx is None or path_idx is None:
                return False

            desc_k32 = data[import_off + k32_idx*20 : import_off + (k32_idx+1)*20]
            oft_k32, _, _, name_k32, ft_k32 = struct.unpack("<IIIII", desc_k32)

            desc_path = data[import_off + path_idx*20 : import_off + (path_idx+1)*20]
            oft_path, _, _, _, ft_path = struct.unpack("<IIIII", desc_path)

            # Chunk 1 (GetModuleHandleA, Sleep)
            s1 = struct.unpack_from("<Q", data, rva_to_offset(oft_k32 + 8*8))[0]
            s2 = struct.unpack_from("<Q", data, rva_to_offset(oft_k32 + 9*8))[0]
            ft_c1 = ft_k32 + 8*8

            # Chunk 2 (CloseHandle .. HeapFree)
            c2_syms = []
            for k in range(9):
                s = struct.unpack_from("<Q", data, rva_to_offset(oft_path + (2 + k)*8))[0]
                c2_syms.append(s)
            ft_c2 = ft_path + 2*8

            sec7 = sec_table_offset + 7 * 40
            vsize, vrva, rsize, roff = struct.unpack_from("<IIII", data, sec7 + 8)

            # Write chunk 1 OFT in slack
            c1_oft_rva = vrva + 0x44d0
            c1_oft_off = roff + 0x44d0
            struct.pack_into("<QQQ", data, c1_oft_off, s1, s2, 0)

            # Write chunk 2 OFT in slack
            c2_oft_rva = vrva + 0x4500
            c2_oft_off = roff + 0x4500
            for idx, s in enumerate(c2_syms):
                struct.pack_into("<Q", data, c2_oft_off + idx * 8, s)
            struct.pack_into("<Q", data, c2_oft_off + 9 * 8, 0)

            # Write descriptor 33 and 34
            desc33_off = import_off + 33 * 20
            struct.pack_into("<IIIII", data, desc33_off, c1_oft_rva, 0, 0, name_k32, ft_c1)
            desc34_off = import_off + 34 * 20
            struct.pack_into("<IIIII", data, desc34_off, c2_oft_rva, 0, 0, name_k32, ft_c2)
            desc35_off = import_off + 35 * 20
            struct.pack_into("<IIIII", data, desc35_off, 0, 0, 0, 0, 0)

            # Increase .idata VirtualSize
            struct.pack_into("<I", data, sec7 + 8, 0x4580)

            f.seek(0)
            f.write(data)
            print("Successfully dynamic-patched PE:", exe_path)
            return True
    except Exception as e:
        print("Error patching", exe_path, e)
        return False

def patch_all():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    patterns = [
        os.path.join(base_dir, "target", "*", "lake-controller.exe"),
        os.path.join(base_dir, "target", "*", "deps", "lake_controller-*.exe"),
    ]
    for pat in patterns:
        for f in glob.glob(pat):
            patch_file(f)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        patch_file(sys.argv[1])
    else:
        patch_all()
