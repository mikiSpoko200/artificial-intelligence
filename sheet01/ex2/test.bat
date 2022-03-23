@echo off
call ..\..\.venv\Scripts\activate
py ..\validator.py zad2 py main.py
call deactivate
pause