pub(crate) const fn number_len(number: u64) -> u8 {
    if number == 0 {
        return 0;
    }

    let mut number_length = 1;
    let mut remaining_number = number;

    while remaining_number >= 10 {
        number_length += 1;
        remaining_number /= 10;
    }

    number_length
}
