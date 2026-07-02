/*
Task

Mr.Right always tell the truth, Mr.Wrong always tell the lies.

Some people are queuing to buy movie tickets, and one of them is Mr.Wrong. Everyone else is Mr.Right. Please judge who is Mr.Wrong according to their conversation.

[[Input]] A string array: conversation

They always talking about I'm in ... position., The man behind me is ... ., The man in front of me is ... ., There are/is ... people in front of me., There are/is ... people behind me..

Please note that everyone has at least one sentence and only one people is Mr.Wrong ;-)

[[Output]] A string: The name of Mr.Wrong. If can not judge, return null (when several valid solutions are possible).

Examples:

conversation=[
"John:I'm in 1st position.",
"Peter:I'm in 2nd position.",
"Tom:I'm in 1st position.",
"Peter:The man behind me is Tom."
]
findOutMrWrong(conversation) should return "Tom"

conversation=[
"John:I'm in 1st position.",
"Peter:I'm in 2nd position.",
"Tom:I'm in 1st position.",
"Peter:The man in front of me is Tom."
]
findOutMrWrong(conversation) should return "John"

conversation=[
"John:I'm in 1st position.",
"Peter:There is 1 people in front of me.",
"Tom:There are 2 people behind me.",
"Peter:The man behind me is Tom."
]
findOutMrWrong(conversation) should return "Tom"

const conversation=[
"John:The man behind me is Peter.",
"Peter:There is 1 people in front of me.",
"Tom:There are 2 people behind me.",
"Peter:The man behind me is Tom."
]
findOutMrWrong(conversation) should return null
Two solutions are possible in the last example: 1) Peter is Mr.Wrong and the order is Tom, John, Peter; 2) Tom is Mr.Wrong and the order is John, Peter, Tom. In this case, the result is null.

Random Tests

n
n is the number of people.

500 tests with
n
=
4
n=4 and with one sentence for each person

100 tests with
3
≤
n
≤
10
3≤n≤10

100 tests with
3
≤
n
≤
10
3≤n≤10 where all sentences are in the form The man behind me is ... ., The man in front of me ... .

100 tests with
10
≤
n
≤
20
10≤n≤20

100 tests with
30
≤
n
≤
40
30≤n≤40

*/
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Statement<'a> {
    MyPosition(usize), // 1-indexed
    BehindMe(&'a str),
    InFrontOfMe(&'a str),
    PeopleInFront(usize),
    PeopleBehind(usize),
}

fn parse_statement<'a>(s: &'a str) -> Option<Statement<'a>> {
    let s = s.trim();
    if s.ends_with('.') {
        let s = &s[..s.len() - 1];
        if s.starts_with("I'm in ") {
            let rest = s.strip_prefix("I'm in ")?;
            let num_part = rest.trim_end_matches(" position");
            let num = num_part
                .trim_end_matches("st")
                .trim_end_matches("nd")
                .trim_end_matches("rd")
                .trim_end_matches("th")
                .parse()
                .ok()?;
            return Some(Statement::MyPosition(num));
        }
        if s.starts_with("The man behind me is ") {
            let name = s.strip_prefix("The man behind me is ")?.trim();
            return Some(Statement::BehindMe(name));
        }
        if s.starts_with("The man in front of me is ") {
            let name = s.strip_prefix("The man in front of me is ")?.trim();
            return Some(Statement::InFrontOfMe(name));
        }
        if s.starts_with("There is ") {
            let rest = s.strip_prefix("There is ")?;
            if let Some(num_part) = rest.strip_suffix(" people in front of me") {
                let num: usize = num_part.trim().parse().ok()?;
                return Some(Statement::PeopleInFront(num));
            }
            if let Some(num_part) = rest.strip_suffix(" people behind me") {
                let num: usize = num_part.trim().parse().ok()?;
                return Some(Statement::PeopleBehind(num));
            }
        }
        if s.starts_with("There are ") {
            let rest = s.strip_prefix("There are ")?;
            if let Some(num_part) = rest.strip_suffix(" people in front of me") {
                let num: usize = num_part.trim().parse().ok()?;
                return Some(Statement::PeopleInFront(num));
            }
            if let Some(num_part) = rest.strip_suffix(" people behind me") {
                let num: usize = num_part.trim().parse().ok()?;
                return Some(Statement::PeopleBehind(num));
            }
        }
    }
    None
}

fn parse_conversation<'a>(conversation: &[&'a str]) -> HashMap<&'a str, Vec<Statement<'a>>> {
    let mut result: HashMap<&str, Vec<Statement>> = HashMap::new();
    for line in conversation {
        if let Some((name, content)) = line.split_once(':') {
            let name = name.trim();
            if let Some(stmt) = parse_statement(content) {
                result.entry(name).or_default().push(stmt);
            }
        }
    }
    result
}

fn statement_true(stmt: &Statement, speaker: &str, order: &[&str]) -> bool {
    let pos = match order.iter().position(|&x| x == speaker) {
        Some(p) => p,
        None => return false,
    };
    let n = order.len();
    match stmt {
        Statement::MyPosition(k) => pos + 1 == *k,
        Statement::BehindMe(other) => pos + 1 < n && order[pos + 1] == *other,
        Statement::InFrontOfMe(other) => pos > 0 && order[pos - 1] == *other,
        Statement::PeopleInFront(k) => pos == *k,
        Statement::PeopleBehind(k) => n - 1 - pos == *k,
    }
}

fn statement_false(
    stmt: &Statement,
    speaker: &str,
    order: &[&str],
) -> bool {
    !statement_true(stmt, speaker, order)
}

fn get_all_people<'a>(parsed: &HashMap<&'a str, Vec<Statement<'a>>>) -> Vec<&'a str> {
    let mut people: std::collections::HashSet<&'a str> = std::collections::HashSet::new();
    for name in parsed.keys() {
        people.insert(*name);
    }
    for stmts in parsed.values() {
        for stmt in stmts {
            match stmt {
                Statement::BehindMe(n) | Statement::InFrontOfMe(n) => {
                    people.insert(*n);
                }
                _ => {}
            }
        }
    }
    people.into_iter().collect()
}

fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let mut result = vec![];
    for i in 0..items.len() {
        let mut rest = items.to_vec();
        let item = rest.remove(i);
        for mut perm in permutations(&rest) {
            perm.insert(0, item.clone());
            result.push(perm);
        }
    }
    result
}

pub fn find_out_mr_wrong<'a>(conversation: &[&'a str]) -> Option<&'a str> {
    let parsed = parse_conversation(conversation);
    let people = get_all_people(&parsed);

    if people.is_empty() {
        return None;
    }

    let mut valid_wrong: Vec<&'a str> = vec![];

    for &candidate in &people {
        let truth_teller_stmts: Vec<(&str, &Statement)> = parsed
            .iter()
            .filter(|(name, _)| **name != candidate)
            .flat_map(|(name, stmts)| stmts.iter().map(|s| (*name, s)))
            .collect();

        let liar_stmts: Vec<&Statement> = parsed
            .get(candidate)
            .map(|v| v.iter().collect())
            .unwrap_or_default();

        let found = permutations(&people).into_iter().any(|order| {
            let order_refs: Vec<&str> = order.iter().map(|s| *s).collect();

            let all_truth = truth_teller_stmts
                .iter()
                .all(|(speaker, stmt)| statement_true(stmt, speaker, &order_refs));

            let all_lie = liar_stmts
                .iter()
                .all(|stmt| statement_false(stmt, candidate, &order_refs));

            all_truth && all_lie
        });

        if found {
            valid_wrong.push(candidate);
        }
    }

    if valid_wrong.len() == 1 {
        Some(valid_wrong[0])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        for (conversation, expected) in SAMPLE_TEST_CASES {
            let actual = find_out_mr_wrong(conversation);
            assert_eq!(actual, expected);
        }
    }

    const SAMPLE_TEST_CASES: [(&[&str], Option<&str>); 10] = [
        (
            &[
                "John:I'm in 1st position.",
                "Peter:I'm in 2nd position.",
                "Tom:I'm in 1st position.",
                "Peter:The man behind me is Tom.",
            ],
            Some("Tom"),
        ),
        (
            &[
                "John:I'm in 1st position.",
                "Peter:I'm in 2nd position.",
                "Tom:I'm in 1st position.",
                "Peter:The man in front of me is Tom.",
            ],
            Some("John"),
        ),
        (
            &[
                "John:I'm in 1st position.",
                "Peter:There is 1 people in front of me.",
                "Tom:There are 2 people behind me.",
                "Peter:The man behind me is Tom.",
            ],
            Some("Tom"),
        ),
        (
            &[
                "John:The man behind me is Peter.",
                "Peter:There is 1 people in front of me.",
                "Tom:There are 2 people behind me.",
                "Peter:The man behind me is Tom.",
            ],
            None,
        ),
        (
            &[
                "Dowfls:There is 0 people behind me.",
                "Dowfls:I'm in 4th position.",
                "Ljiyxbmr:I'm in 2nd position.",
                "Ljiyxbmr:There is 1 people in front of me.",
                "Cvvugb:There are 2 people in front of me.",
                "Cvvugb:There is 1 people behind me.",
                "Tzjlvruhk:The man behind me is Dowfls.",
                "Tzjlvruhk:There are 2 people in front of me.",
            ],
            None,
        ),
        (
            &[
                "Tom:The man behind me is Bob.",
                "Bob:The man in front of me is Tom.",
                "Bob:The man behind me is Gary.",
                "Gary:The man in front of me is Bob.",
                "Fred:I'm in 1st position.",
            ],
            Some("Fred"),
        ),
        (&["Wrong:The man behind me is Wrong."], Some("Wrong")),
        (
            &[
                "Charles:The man behind me is Gavin.",
                "Gavin:I'm in 1st position.",
                "Ken:The man in front of me is Gavin.",
                "Charles:The man in front of me is Gavin.",
            ],
            Some("Charles"),
        ),
        (
            &[
                "Greg:I'm in 1st position.",
                "Daniel:There are 2 people in front of me.",
                "Ramone:I'm in 3rd position.",
                "Daniel:There are 2 people behind me.",
            ],
            Some("Daniel"),
        ),
        (
            &[
                "Frodo:I'm in 3rd position.",
                "Gollum:I'm in 3rd position.",
                "Sam:The man behind me is Frodo.",
                "Gollum:The man behind me is Frodo.",
            ],
            Some("Gollum"),
        ),
    ];
}
