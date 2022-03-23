@echo off
call ..\..\.venv\Scripts\activate
py ../validator.py zad6 py main.py
call deactivate
pause