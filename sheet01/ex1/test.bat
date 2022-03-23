@echo off
call ..\..\.venv\Scripts\activate
py ..\validator.py zad1 main.exe
call deactivate
pause