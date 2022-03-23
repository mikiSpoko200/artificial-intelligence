@echo off
call ..\..\.venv\Scripts\activate
py ..\validator.py zad4 py main.py
call deactivate
pause