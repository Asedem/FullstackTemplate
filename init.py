#! /usr/bin/env python3

import os


def read_string(prompt: str) -> str:
    return input(prompt)


def read_yesno(prompt: str) -> bool:
    while True:
        inp = input(f'{prompt} (y)es/(n)o: ')
        if inp == 'y': return True
        if inp == 'n': return False


def replace_in_file(file_path, old, new, count = -1):
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        new_content = content.replace(old, new, count if count != -1 else -1)
        with open(file_path, 'w') as f:
            f.write(new_content)
    except FileNotFoundError:
        print(f"Error: The file '{file_path}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")


def replace_with_message(project_part, file_path, old, new):
    print(f'Replacing names in {project_part} ({file_path})')
    replace_in_file(f'.{project_part}/{file_path}', old, new)

# Main Logic

proj_name = read_string('Enter a name for the Project: ')

proj_id = proj_name.lower().replace(' ', '_')
if not read_yesno(f'Would you like {proj_id} as your project id?'):
    proj_id = read_string('Enter a project id: ')

db_name = proj_id
if not read_yesno('Would you like to use the project id as your database name?'):
    db_name = read_string('Enter a name for your database: ')

comp_file = 'docker-compose.yml'

replace_with_message('/database', 'create_database.sql', 'x', db_name)
replace_with_message('/frontend', 'index.html', 'Fullstack template', proj_name)
replace_with_message('/', comp_file, 'x_internal_network', f'{proj_id}_internal_network')
replace_with_message('/', comp_file, 'x_frontend', f'{proj_id}_frontend')
replace_with_message('/', comp_file, 'x_database', f'{proj_id}_database')
replace_with_message('/', comp_file, 'x_backend', f'{proj_id}_backend')
replace_with_message('/', comp_file, '5432/x', f'5432/{proj_id}')


try:
    os.remove('./init.py')
    print('Project sucessfully created')
except FileNotFoundError:
    print('Project sucessfully created. Please remove the init.py file now')
