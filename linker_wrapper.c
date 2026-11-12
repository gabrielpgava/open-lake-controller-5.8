#include <windows.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

static void run_patch(void) {
    wchar_t cmd[2048];
    swprintf(cmd, 2048, L"\"C:\\Python314\\python.exe\" \"C:\\Users\\gabri\\Documents\\Craft Anything\\lake-controller\\app\\patch_pe.py\"");

    STARTUPINFOW si;
    PROCESS_INFORMATION pi;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    if (CreateProcessW(NULL, cmd, NULL, NULL, TRUE, 0, NULL, NULL, &si, &pi)) {
        WaitForSingleObject(pi.hProcess, INFINITE);
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
}

int wmain(int argc, wchar_t *argv[]) {
    const wchar_t *real_gcc = L"C:\\Users\\gabri\\AppData\\Local\\Microsoft\\WinGet\\Packages\\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\\mingw64\\bin\\gcc.exe";
    
    const wchar_t *cmdline = GetCommandLineW();
    
    const wchar_t *args = cmdline;
    if (*args == L'\"') {
        args++;
        while (*args && *args != L'\"') args++;
        if (*args == L'\"') args++;
    } else {
        while (*args && *args != L' ' && *args != L'\t') args++;
    }
    while (*args == L' ' || *args == L'\t') args++;

    size_t new_cmd_len = wcslen(real_gcc) + wcslen(args) + 16;
    wchar_t *new_cmd = (wchar_t *)malloc(new_cmd_len * sizeof(wchar_t));
    swprintf(new_cmd, new_cmd_len, L"\"%ls\" %ls", real_gcc, args);

    STARTUPINFOW si;
    PROCESS_INFORMATION pi;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    if (!CreateProcessW(NULL, new_cmd, NULL, NULL, TRUE, 0, NULL, NULL, &si, &pi)) {
        free(new_cmd);
        return 1;
    }

    WaitForSingleObject(pi.hProcess, INFINITE);
    DWORD exit_code = 0;
    GetExitCodeProcess(pi.hProcess, &exit_code);
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
    free(new_cmd);

    if (exit_code == 0) {
        run_patch();
    }

    return (int)exit_code;
}
