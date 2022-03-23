@echo off
call ..\..\.venv\Scripts\activate
py ../validator.py zad8 py main.py
call deactivate
pause