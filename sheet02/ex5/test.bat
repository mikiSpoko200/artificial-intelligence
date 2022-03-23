@echo off
call ..\..\.venv\Scripts\activate
py ../validator.py zad5 py main.py
call deactivate
pause