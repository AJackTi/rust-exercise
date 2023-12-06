pub struct Company {
    ceo: String,
    receptionist: String,
    marketing: String,
}

pub struct CompanyIter<'a> {
    c: &'a Company,
    n: i32,
}

impl<'a> Iterator for CompanyIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        match self.n {
            1 => Some(&self.c.ceo),
            2 => Some(&self.c.receptionist),
            3 => Some(&self.c.marketing),
            _ => None,
        }
    }
}

impl<'a> IntoIterator for &'a Company {
    type IntoIter = CompanyIter<'a>;
    type Item = &'a str;
    fn into_iter(self) -> Self::IntoIter {
        CompanyIter { c: self, n: 0 }
    }
}

#[cfg(test)]
mod tests_company {
    use super::*;

    #[test]
    fn test_into_iter() {
        let c = Company {
            ceo: "John".to_string(),
            receptionist: "Jane".to_string(),
            marketing: "Joe".to_string(),
        };

        let mut res = String::new();
        for m in &c {
            res.push_str(m);
        }

        assert_eq!(res, "JohnJaneJoe");
    }
}
