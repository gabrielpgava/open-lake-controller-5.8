@echo off
setlocal EnableDelayedExpansion
title Instalador - Open DolbyLake Software v0.0.1
color 1F

echo ==============================================================================
echo                 OPEN DOLBYLAKE SOFTWARE - INSTALADOR v0.0.1
echo ==============================================================================
echo.
echo [AVISO IMPORTANTE]
echo Esta e uma versao experimental (v0.0.1) que AINDA NAO FOI TESTADA em
echo equipamento fisico real (Dolby Lake Processor / LM / PLM).
echo.
pause

set "INSTALL_DIR=%LocalAppData%\Programs\OpenDolbyLake"
echo.
echo [*] Criando pasta de instalacao: "%INSTALL_DIR%"...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo [*] Copiando arquivos do programa...
copy /Y "open-dolbylake-software.exe" "%INSTALL_DIR%\open-dolbylake-software.exe" >nul
if exist "README.md" copy /Y "README.md" "%INSTALL_DIR%\" >nul
if exist "MECHANICS.md" copy /Y "MECHANICS.md" "%INSTALL_DIR%\" >nul

echo [*] Criando atalho na Area de Trabalho...
set "DESKTOP_DIR=%USERPROFILE%\Desktop"
powershell -Command "$ws = New-Object -ComObject WScript.Shell; $s = $ws.CreateShortcut('%DESKTOP_DIR%\Open DolbyLake Software.lnk'); $s.TargetPath = '%INSTALL_DIR%\open-dolbylake-software.exe'; $s.WorkingDirectory = '%INSTALL_DIR%'; $s.Description = 'Open DolbyLake Software v0.0.1'; $s.Save()"

echo [*] Criando atalho no Menu Iniciar...
set "START_DIR=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Open DolbyLake"
if not exist "%START_DIR%" mkdir "%START_DIR%"
powershell -Command "$ws = New-Object -ComObject WScript.Shell; $s = $ws.CreateShortcut('%START_DIR%\Open DolbyLake Software.lnk'); $s.TargetPath = '%INSTALL_DIR%\open-dolbylake-software.exe'; $s.WorkingDirectory = '%INSTALL_DIR%'; $s.Description = 'Open DolbyLake Software v0.0.1'; $s.Save()"

echo.
echo ==============================================================================
echo [SUCESSO] O Open DolbyLake Software v0.0.1 foi instalado com sucesso!
echo Atalho criado na sua Area de Trabalho.
echo ==============================================================================
echo.
set /p RUNNOW="Deseja abrir o Open DolbyLake Software agora? (S/N): "
if /i "%RUNNOW%"=="S" (
    start "" "%INSTALL_DIR%\open-dolbylake-software.exe"
)
exit /b 0
