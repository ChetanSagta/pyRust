// atom      ::=  identifier | literal | enclosure
pub enum Atom{
    identifier,
    literal,
    enclosure
}

// enclosure ::=  parenth_form | list_display | dict_display | set_display
               // | generator_expression | yield_atom
pub enum Enclosure{
    parenth_form,
    list_display,
    dict_display,
    set_display,
    generator_expression,
    yield_atom
}
