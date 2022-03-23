@echo off
call ..\..\.venv\Scripts\activate
py ../validator.py zad7 py main.py
call deactivate
pause