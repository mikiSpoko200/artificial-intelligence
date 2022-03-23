@echo off
call ..\..\.venv\Scripts\activate
py ../validator.py zad1 py main.py
call deactivate
pause