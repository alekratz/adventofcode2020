import sys
import re


with sys.stdin as fp:
    inputs = fp.read().split("\n\n")

pat = re.compile("(\S+):(\S+)")
req = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

print("got", len(inputs), "passports")

count = 0
for ident in inputs:
    items = pat.findall(ident)
    item_keys = {key for (key, _) in items}

    input_valid = item_keys & req == req
    pid = ''
    for (key, val) in pat.findall(ident):
        if key == 'byr':
            valid = 1920 <= int(val) <= 2002
        elif key == 'iyr':
            valid = 2010 <= int(val) <= 2020
        elif key == 'eyr':
            valid = 2020 <= int(val) <= 2030
        elif key == 'hgt':
            match = re.match(r"(\d+)(in|cm)?", val) or ('', '0', '')
            valid = (59 <= int(match[1]) <= 76 and match[2] == 'in') or (150 <= int(match[1]) <= 193 and match[2] == 'cm')
        elif key == 'hcl':
            valid = bool(re.match(r"#[a-f0-9]{6}", val))
        elif key == 'ecl':
            valid = val in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth')
        elif key == 'pid':
            valid = bool(re.match(r"\d{9}", val))
            pid = val
        elif key == 'cid':
            valid = True
        input_valid &= valid
    if input_valid:
        print("pid:" + pid)
        count += 1

print(count, "valid entries")
