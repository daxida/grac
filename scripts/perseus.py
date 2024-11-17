"""Convert a perseus xml to raw text.

The xmls can be found at:
https://github.com/PerseusDL/canonical-greekLit/tree/master/data

Copy them to a ".xml" file then run this giving it the xml path.
"""

import xml.etree.ElementTree as ET
import argparse


def extract_text_from_xml(file_path):
    try:
        tree = ET.parse(file_path)
        root = tree.getroot()
        namespace = {"tei": "http://www.tei-c.org/ns/1.0"}
        lines = []
        for line in root.findall(".//tei:l", namespaces=namespace):
            if line.text:
                lines.append(line.text.strip())
        return "\n".join(lines)
    except Exception as e:
        print(f"Error processing the XML file: {e}")
        return ""


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", type=str)
    parser.add_argument("output_file", type=str)
    args = parser.parse_args()

    raw_text = extract_text_from_xml(args.input_file)
    print(f"Parsed {len(raw_text.splitlines())} lines")

    with open(args.output_file, "w", encoding="utf-8") as output_file:
        output_file.write(raw_text)

    print(f"Extracted text saved to {args.output_file}")


if __name__ == "__main__":
    main()
