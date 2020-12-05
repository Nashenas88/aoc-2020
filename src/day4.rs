use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default, Clone)]
pub struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn has_necessary_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_RE: Regex = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PASSPORT_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        if !matches!(self.birth_year,
                Some(year) if year >= 1920 && year <= 2002)
        {
            return false;
        }

        if !matches!(self.issue_year,
                Some(year) if year >= 2010 && year <= 2020)
        {
            return false;
        }

        if !matches!(self.expiration_year,
                Some(year) if year >= 2020 && year <= 2030)
        {
            return false;
        }

        if !self
            .height
            .as_deref()
            .and_then(|height| HEIGHT_RE.captures_iter(height).next())
            .map(|captures| {
                let value: u8 = match captures[1].parse() {
                    Ok(val) => val,
                    _ => return false,
                };
                let units = &captures[2];
                match (value, units) {
                    (value, "cm") if value >= 150 && value <= 193 => true,
                    (value, "in") if value >= 59 && value <= 76 => true,
                    _ => false,
                }
            })
            .unwrap_or(false)
        {
            return false;
        }

        if !self
            .hair_color
            .as_deref()
            .map(|hair_color| HAIR_RE.is_match(hair_color))
            .unwrap_or(false)
        {
            return false;
        }

        if !self
            .eye_color
            .as_deref()
            .map(|eye_color| EYE_RE.is_match(eye_color))
            .unwrap_or(false)
        {
            return false;
        }

        if !self
            .passport_id
            .as_deref()
            .map(|passport_id| PASSPORT_RE.is_match(passport_id))
            .unwrap_or(false)
        {
            return false;
        }

        true
    }

    fn parse_fields<'a, I>(&mut self, mut params: I)
    where
        I: Iterator<Item = &'a str>,
    {
        let key = if let Some(key) = params.next() {
            key
        } else {
            return;
        };

        let value = if let Some(value) = params.next() {
            value
        } else {
            return;
        };

        // TODO: Handle another param = failure?

        match key {
            "byr" => self.birth_year = Some(value.parse().unwrap()),
            "iyr" => self.issue_year = Some(value.parse().unwrap()),
            "eyr" => self.expiration_year = Some(value.parse().unwrap()),
            "hgt" => self.height = Some(value.into()),
            "hcl" => self.hair_color = Some(value.into()),
            "ecl" => self.eye_color = Some(value.into()),
            "pid" => self.passport_id = Some(value.into()),
            "cid" => self.country_id = Some(value.into()),
            _ => {
                println!("WTF is {}", key);
            } // no-op, TODO: Log error?
        }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .replace("\n", " ")
        .split("  ")
        .map(|line| {
            let mut passport = <Passport as Default>::default();
            for kv in line.split_ascii_whitespace() {
                passport.parse_fields(kv.split(':'));
            }

            passport
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> u32 {
    input
        .as_ref()
        .iter()
        .map(|p| p.has_necessary_fields() as u32)
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Passport]) -> u32 {
    input.as_ref().iter().map(|p| p.is_valid() as u32).sum()
}

#[test]
fn test_present() {
    let contents = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
    let input = input_generator(contents);
    let result = solve_part1(&input);
    assert_eq!(result, 2);
}

#[test]
fn test_invalid() {
    let contents = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

byr:1919 iyr:2015 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:2003 iyr:2015 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2009 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2021 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2019 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2031 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:149cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:194cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:58in hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:77in hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#55555 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#7777777 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#666666 ecl:abc pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#666666 ecl:amb pid:01234567

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#666666 ecl:amb pid:0123456789
";
    let input = input_generator(contents);
    let result = solve_part2(&input);
    assert_eq!(result, 0);
}

#[test]
fn test_valid() {
    let contents = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

byr:1920 iyr:2015 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:2002 iyr:2015 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2010 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2020 eyr:2025 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2020 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2030 hgt:180cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:150cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:193cm hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:59in hcl:#666666 ecl:amb pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:76in hcl:#666666 ecl:blu pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#012345 ecl:brn pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#6789ab ecl:gry pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#cdef01 ecl:grn pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#666666 ecl:hzl pid:012345678

byr:1950 iyr:2015 eyr:2025 hgt:60in hcl:#666666 ecl:oth pid:987654321
";
    let input = input_generator(contents);
    let result = solve_part2(&input);
    assert_eq!(result, 19);
}
