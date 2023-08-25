# Practice Book: Aditya Bbhargava: Grokking Algorithms, 2nd Edition

This repository contains my practical exercises from the upcomming book [«Grokking Algorithms, 2nd Edition» by Aditya Y. Bhargava (Manning)](https://www.manning.com/books/grokking-algorithms-second-edition).

![Book Cover](cover.png)

The original code repository is located [here in GitHub](https://github.com/egonSchiele/grokking_algorithms).


## Installation Setup on Ubuntu Linux

If you wish, you can follow these installation steps for **Linux Ubuntu 23.04**.

### Python Installation

Here, [pyenv](https://github.com/pyenv/pyenv) is used to create a shared virtual environment for all projects and [pip-tools](https://github.com/jazzband/pip-tools) to install libraries.

#### Install pyenv:

Install dependencies:

```shell
$ sudo aptitude update; sudo apt install curl build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev curl libncursesw5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev -y;
```

Install `pyenv`:

```shell
$ curl https://pyenv.run | bash;
```

Then add these lines to your `.bashrc` file:

```shell
export PATH="$HOME/.pyenv/bin:$PATH"
eval "$(pyenv init --path)"
eval "$(pyenv virtualenv-init -)"
```

Verify `pyenv` is working:
    
```shell
$ pyenv --help
```

#### Install a Python version

```shell
$ pyenv install 3.11.2;
``` 

#### Create the Python Virtual Environment

Move to your working directory where you will develop the projects:

```shell
$ pyenv local 3.11.2;
$ pyenv virtualenv grokking-algorithms;
$ pyenv local 3.11.2/envs/grokking-algorithms;
$ pyenv activate 3.11.2/envs/grokking-algorithms;
```

#### Install Python Dependencies

Install `pip-tools` and then the project dependencies.

```shell
$ pip install --upgrade pip;
$ python -m pip install pip-tools --upgrade;
$ pip-compile requirements.in --upgrade;
$ pip install -r requirements.txt;

