use util::push_action;

const TICTACTOE_ACTIONS: &[(&str, &str, &str)] = &[
    ("create", "[\"alice\",\"bob\"]", "alice@active"),
    ("makemove", "[\"alice\",\"bob\",1,0,0]", "alice@active"),
    ("makemove", "[\"alice\",\"bob\",2,1,0]", "bob@active"),
    ("makemove", "[\"alice\",\"bob\",1,0,1]", "alice@active"),
    ("makemove", "[\"alice\",\"bob\",2,1,1]", "bob@active"),
    ("makemove", "[\"alice\",\"bob\",1,0,2]", "alice@active"),
    ("restart", "[\"alice\",\"bob\",1]", "alice@active"),
    ("makemove", "[\"alice\",\"bob\",1,0,0]", "alice@active"),
    ("makemove", "[\"alice\",\"bob\",2,1,0]", "bob@active"),
    ("makemove", "[\"alice\",\"bob\",1,0,1]", "alice@active"),
    ("makemove", "[\"alice\",\"bob\",2,1,1]", "bob@active"),
    ("makemove", "[\"alice\",\"bob\",1,0,2]", "alice@active"),
    ("close", "[\"alice\",\"bob\"]", "alice@active"),
];

pub fn run_examples() {
    push_action("hello", "hi", "[\"contributor\"]", "hello@active");
    for (action, data, auth) in TICTACTOE_ACTIONS {
        push_action("tictactoe", action, data, auth);
    }
}
