@echo off
call ..\..\.venv\Scripts\activate
py ../validator.py zad3 py main.py
call deactivate
pause