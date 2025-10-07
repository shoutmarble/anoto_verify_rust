
>  `INSTALL PYTHON VENV` to add `PYTHON` dependences

> >`python3 -m venv venv_md`

> Enable the Python "Virtual Environment"

> > `. venv_md/bin/activate`

> INSTALL PYTHON DEPENDENCIES
> > cd requirements/
> > pip install -r requirements.txt

> Install necessary tools for `py-microdots`
> > pip install setuptools

> BUILD AND INSTALL `py-microdots`
> > python3 setup.py build
> > python3 setup.py install

> RUN `py-microdots` EXAMPLES

> cd examples/
> > python3 hello_anoto.py
> > python3 verify_anoto.py 

`python3 verify_anoto.py`
```
G shape: (40, 43), section: (57, 8)
G shape: (36, 37), section: (50, 62)
G shape: (42, 70), section: (52, 42)
G shape: (68, 67), section: (47, 6)
G shape: (30, 28), section: (62, 32)
G shape: (45, 65), section: (37, 43)
G shape: (58, 32), section: (24, 65)
G shape: (49, 43), section: (51, 56)
G shape: (64, 33), section: (31, 33)
G shape: (52, 53), section: (26, 70)
G shape: (9, 16), section: (10, 2)
pos: (np.int64(7), np.int64(3)) sec: (np.int64(10), np.int64(2)) rot: 0
```