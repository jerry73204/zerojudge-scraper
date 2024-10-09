import os
import argparse
from urllib.parse import urlparse, parse_qs
from pathlib import Path

import pywebcopy
from selenium.webdriver import Chrome
from selenium.webdriver.common.by import By


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--list_url", default="https://zerojudge.tw/Problems?tag=APCS")
    parser.add_argument("--output_dir", default="data")
    args = parser.parse_args()

    # Create the output directory
    output_dir = Path(args.output_dir)
    output_dir.mkdir()

    # Visit the problem list page
    driver = Chrome()
    driver.get(args.list_url)
    link_elems = driver.find_elements(
        By.XPATH, "/html/body/div[3]/div/table/tbody/tr[*]/td[3]/a"
    )
    links = list(elem.get_attribute("href") for elem in link_elems)
    driver.quit()

    # Visit each problem page one by one
    project_dir = str(output_dir.resolve())

    for link in links:
        print(f"Downloading {link}")
        url = urlparse(link)
        problem_id = parse_qs(url.query)["problemid"][0]

        pywebcopy.save_website(
            url=link,
            project_folder=project_dir,
            project_name=problem_id,
            threaded=True,
        )

    return 0
