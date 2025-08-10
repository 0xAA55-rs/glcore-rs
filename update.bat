@echo off
cd glcorecpph
python glparse.py
copy glcore.rs ..\src\
