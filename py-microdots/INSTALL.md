python3 -m venv venv_md

. venv_md/bin/activate

pip install setuptools

cd requirements/
pip install -r requirements.txt

python3 setup.py build
python3 setup.py install

cd examples/
python3 hello_anoto.py
