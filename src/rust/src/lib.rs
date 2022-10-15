use extendr_api::prelude::*;
pub mod helpers;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Read a text file out and return a string
/// @export
/// @examples
/// read_file_to_string("test_file1.txt")
#[extendr]
pub fn read_file_to_string(file_path: String) -> String {
    return rust_miscellaneous_pkg::basic_file_operation::basic_file_operation::read_file_to_string(
        file_path,
    );
}

#[extendr]
pub fn write_file_from_char_vec(file_path: String, input_vec: Vec<String>, append: bool) -> bool {
    return rust_miscellaneous_pkg::basic_file_operation::basic_file_operation::write_file_from_char_vec(file_path,input_vec,append);
}

/// Get hamming Distance of two strings
/// @export
/// @examples hamming('hallo welt','hallo test')
#[extendr]
pub fn hamming(alpha_str: String, beta_str: String) -> u64 {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::hamming(alpha_str, beta_str);
}

/// Get levenshtein distance of two strings
/// @export
/// @examples levenshtein('hallo welt', 'hallo test')
#[extendr]
pub fn levenshtein(alpha_str: String, beta_str: String) -> u32 {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::levenshtein(
        alpha_str, beta_str,
    );
}

/// levenshtein_smid matching of two strings
/// @export
/// @examples levenshtein_smid('hallo welt', 'hallo test')
#[extendr]
pub fn levenshtein_smid(alpha_str: String, beta_str: String) -> u32 {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::levenshtein_smid(
        alpha_str, beta_str,
    );
}

/// hamming_smid matching of two strings
/// @export
/// @examples hamming_smid('hallo welt', 'hallo test')
#[extendr]
pub fn hamming_smid(alpha_str: String, beta_str: String) -> u64 {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::hamming_smid(
        alpha_str, beta_str,
    );
}

/// bom matching of two strings
/// @export
/// @examples bom_matching('hallo welt', 'hallo test')
#[extendr]
pub fn bom_matching(pattern: String, text: String) -> Vec<usize> {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::bom_matching(pattern, text);
}

/// bndm matching of two strings
/// @export
/// @examples bndm_matching('hallo welt', 'hallo test')
#[extendr]
pub fn bndm_matching(pattern: String, text: String) -> Vec<usize> {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::bndm_matching(pattern, text);
}

/// horspool matching of two strings
/// @export
/// @examples horspool_matching('hallo welt', 'hallo test')
#[extendr]
pub fn horspool_matching(pattern: String, text: String) -> Vec<usize> {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::horspool_matching(
        pattern, text,
    );
}

/// kmp matching of two strings
/// @export
/// @examples kmp_matching('hallo welt', 'hallo test')
#[extendr]
pub fn kmp_matching(pattern: String, text: String) -> Vec<usize> {
    return rust_miscellaneous_pkg::bio_algorithms::bio_algorithms::kmp_matching(pattern, text);
}

/// Return sha224 hash of a string to R.
/// @export
/// @examples get_sha224("Hallo Welt")
#[extendr]
pub fn get_sha224(input: String) -> String {
    return rust_miscellaneous_pkg::crypto::crypto::get_sha224(bytes::Bytes::from(input));
}

/// Return sha256 hash of a string to R.
/// @export
/// @examples get_sha256("Hallo Welt")
#[extendr]
pub fn get_sha256(input: String) -> String {
    return rust_miscellaneous_pkg::crypto::crypto::get_sha256(bytes::Bytes::from(input));
}

/// Return sha384 hash of a string to R.
/// @export
/// @examples get_sha384("Hallo Welt")
#[extendr]
pub fn get_sha384(input: String) -> String {
    return rust_miscellaneous_pkg::crypto::crypto::get_sha384(bytes::Bytes::from(input));
}

/// Return sha512 hash of an input string to R.
/// @export
/// @examples get_sha512("Hallo Welt")
#[extendr]
pub fn get_sha512(input: String) -> String {
    return rust_miscellaneous_pkg::crypto::crypto::get_sha512(bytes::Bytes::from(input));
}

/// Return sha1 hash of an input string to R.
/// @export
/// @examples
/// get_sha1("Hallo Welt")
#[extendr]
pub fn get_sha1(input: String) -> String {
    return rust_miscellaneous_pkg::crypto::crypto::get_sha1(bytes::Bytes::from(input));
}

/// get random number in a range
/// @export
/// @examples
/// get_random_number_range(0,1000)
#[extendr]
pub fn get_random_number_range(min_num: f64, max_num: f64) -> f64 {
    return rust_miscellaneous_pkg::random_numbers::random_numbers::get_random_number_range(
        min_num, max_num,
    );
}

/// return a fake city name
/// @export
/// @examples
/// get_city()
#[extendr]
pub fn get_city() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_city();
}

/// return a fake country
/// @export
/// @examples
/// get_country()
#[extendr]
pub fn get_country() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_country();
}

/// return a fake country abr
/// @export
/// @examples
/// get_country_abr()
#[extendr]
pub fn get_country_abr() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_country_abr();
}

/// return a fake latitude
/// @export
/// @examples
/// get_latitude()
#[extendr]
pub fn get_latitude() -> f32 {
    return rust_miscellaneous_pkg::faker::faker::get_latitude();
}

/// return a fake longitude
/// @export
/// @examples
/// get_longitude()
#[extendr]
pub fn get_longitude() -> f32 {
    return rust_miscellaneous_pkg::faker::faker::get_longitude();
}

/// return a fake state
/// @export
/// @examples
/// get_state()
#[extendr]
pub fn get_state() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_state();
}

/// return a fake state
/// @export
/// @examples
/// get_street()
#[extendr]
pub fn get_street() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_street();
}

/// return a fake state
/// @export
/// @examples
/// get_street_name()
#[extendr]
pub fn get_street_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_street_name();
}

/// return a fake state number
/// @export
/// @examples
/// get_street_number()
#[extendr]
pub fn get_street_number() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_street_number();
}

/// return a fake state prefix
/// @export
/// @examples
/// get_street_prefix()
#[extendr]
pub fn get_street_prefix() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_street_prefix();
}

/// return fake street suffix
/// @export
/// @examples
/// get_street_suffix()
#[extendr]
pub fn get_street_suffix() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_street_suffix();
}

/// return fake street suffix
/// @export
/// @examples
/// get_zip()
#[extendr]
pub fn get_zip() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_zip();
}

/// return fake animal
/// @export
/// @examples
/// get_anmimal()
#[extendr]
pub fn get_anmimal() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_anmimal();
}

/// return fake cat
/// @export
/// @examples
/// get_cat()
#[extendr]
pub fn get_cat() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_cat();
}

/// return fake dog
/// @export
/// @examples
/// get_dog()
#[extendr]
pub fn get_dog() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_dog();
}

/// return fake farm animal
/// @export
/// @examples
/// get_farm()
#[extendr]
pub fn get_farm() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_farm();
}

/// return fake pet name
/// @export
/// @examples
/// get_pet_name()
#[extendr]
pub fn get_pet_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_pet_name();
}

/// return fake pet name
/// @export
/// @examples
/// get_alcohol()
#[extendr]
pub fn get_alcohol() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_alcohol();
}

/// return fake blg
/// @export
/// @examples
/// get_blg()
#[extendr]
pub fn get_blg() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_blg();
}

/// return fake hop
/// @export
/// @examples
/// get_hop()
#[extendr]
pub fn get_hop() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_hop();
}

/// return fake ibu
/// @export
/// @examples
/// get_ibu()
#[extendr]
pub fn get_ibu() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_ibu();
}

/// return fake malt
/// @export
/// @examples
/// get_malt()
#[extendr]
pub fn get_malt() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_malt();
}

/// return fake beer name
/// @export
/// @examples
/// get_name()
#[extendr]
pub fn get_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_name();
}

/// return fake beer style
/// @export
/// @examples
/// get_style()
#[extendr]
pub fn get_style() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_style();
}
/// return fake beer yeast
/// @export
/// @examples
/// get_yeast()
#[extendr]
pub fn get_yeast() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_yeast();
}

/// return fake bool
/// @export
/// @examples
/// get_bool()
#[extendr]
pub fn get_bool() -> bool {
    return rust_miscellaneous_pkg::faker::faker::get_bool();
}

/// return full color
/// @export
/// @examples
/// get_full_color()
#[extendr]
pub fn get_full_color() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_full_color();
}

/// return hex color
/// @export
/// @examples
/// get_hex_color()
#[extendr]
pub fn get_hex_color() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_hex_color();
}

/// return safe color
/// @export
/// @examples
/// get_safe_color()
#[extendr]
pub fn get_safe_color() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_safe_color();
}

/// return fake company
/// @export
/// @examples
/// get_company()
#[extendr]
pub fn get_company() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_company();
}

/// return buzz word
/// @export
/// @examples
/// get_buzzword()
#[extendr]
pub fn get_buzzword() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_buzzword();
}

/// return bs
/// @export
/// @examples
/// get_bs()
#[extendr]
pub fn get_bs() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_bs();
}

/// return company suffix
/// @export
/// @examples
/// get_company_suffix()
#[extendr]
pub fn get_company_suffix() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_company_suffix();
}

/// return fake email
/// @export
/// @examples
/// get_email()
#[extendr]
pub fn get_email() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_email();
}

/// return fake phone number
/// @export
/// @examples
/// get_phone()
#[extendr]
pub fn get_phone() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_phone();
}

/// return formatted phone number
/// @export
/// @examples
/// get_phone_formatted()
#[extendr]
pub fn get_phone_formatted() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_phone_formatted();
}

/// return currency long
/// @export
/// @examples
/// get_currency_long()
#[extendr]
pub fn get_currency_long() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_currency_long();
}

/// return currency short
/// @export
/// @examples
/// get_currency_short()
#[extendr]
pub fn get_currency_short() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_currency_short();
}

/// return day
/// @export
/// @examples
/// get_day()
#[extendr]
pub fn get_day() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_day();
}

/// return hour
/// @export
/// @examples
/// get_hour()
#[extendr]
pub fn get_hour() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_hour();
}

/// return minute
/// @export
/// @examples
/// get_minute()
#[extendr]
pub fn get_minute() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_minute();
}

/// return secound
/// @export
/// @examples
/// get_secound()
#[extendr]
pub fn get_secound() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_secound();
}

/// return secound
/// @export
/// @examples
/// get_timezone()
#[extendr]
pub fn get_timezone() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_timezone();
}

/// return week day
/// @export
/// @examples
/// get_week_day()
#[extendr]
pub fn get_week_day() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_week_day();
}

/// return year
/// @export
/// @examples
/// get_year()
#[extendr]
pub fn get_year() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_year();
}

/// return file extension
/// @export
/// @examples
/// get_extension()
#[extendr]
pub fn get_extension() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_extension();
}

/// return mime type
/// @export
/// @examples
/// get_mime_type()
#[extendr]
pub fn get_mime_type() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_mime_type();
}

/// return abbreviation
/// @export
/// @examples
/// get_abbreviation()
#[extendr]
pub fn get_abbreviation() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_abbreviation();
}

/// return adjective
/// @export
/// @examples
/// get_adjective()
#[extendr]
pub fn get_adjective() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_adjective();
}

/// return ingverb
/// @export
/// @examples
/// get_ingverb()
#[extendr]
pub fn get_ingverb() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_ingverb();
}

/// add two numbers
/// @export
/// @examples add_num(5, 3)
#[extendr]
pub fn add_num(a: f64, b: f64) -> f64 {
    return rust_miscellaneous_pkg::basic_arithmetic::basic_arithmetic::add_num(a, b);
}

/// sub two numbers
/// @export
/// @examples sub_num(5, 3)
#[extendr]
pub fn sub_num(a: f64, b: f64) -> f64 {
    return rust_miscellaneous_pkg::basic_arithmetic::basic_arithmetic::sub_num(a, b);
}

/// multiply two numbers
/// @export
/// @examples mul_num(5, 3)
#[extendr]
pub fn mul_num(a: f64, b: f64) -> f64 {
    return rust_miscellaneous_pkg::basic_arithmetic::basic_arithmetic::mul_num(a, b);
}

/// div two numbers
/// @export
/// @examples div_num(5, 3)
#[extendr]
pub fn div_num(a: f64, b: f64) -> f64 {
    return rust_miscellaneous_pkg::basic_arithmetic::basic_arithmetic::div_num(a, b);
}

/// calculate the modulo of two numbers
/// @export
/// @examples mod_num(5, 3)
#[extendr]
pub fn mod_num(a: f64, b: f64) -> f64 {
    return rust_miscellaneous_pkg::basic_arithmetic::basic_arithmetic::mod_num(a, b);
}

/// return a fake noun word
/// @export
/// @examples get_noun()
#[extendr]
pub fn get_noun() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_noun();
}

/// return a fake pharse word
/// @export
/// @examples get_pharse()
#[extendr]
pub fn get_pharse() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_pharse();
}

/// return a fake verb
/// @export
/// @examples get_verb()
#[extendr]
pub fn get_verb() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_verb();
}

/// return a hippster word
/// @export
/// @examples get_hippster_word()
#[extendr]
pub fn get_hippster_word() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_hippster_word();
}

/// return a domain name
/// @export
/// @examples get_domain_name()
#[extendr]
pub fn get_domain_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_domain_name();
}

/// return a domain suffix
/// @export
/// @examples get_domain_suffix()
#[extendr]
pub fn get_domain_suffix() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_domain_suffix();
}

/// return a http method
/// @export
/// @examples get_http_method()
#[extendr]
pub fn get_http_method() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_http_method();
}

/// return a ipv4 address
/// @export
/// @examples get_ipv4_address()
#[extendr]
pub fn get_ipv4_address() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_ipv4_address();
}

/// return a ipv6 ddress
/// @export
/// @examples get_ipv6_address()
#[extendr]
pub fn get_ipv6_address() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_ipv6_address();
}

/// return a mac ddress
/// @export
/// @examples get_mac_address()
#[extendr]
pub fn get_mac_address() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_mac_address();
}

/// return a username
/// @export
/// @examples get_username()
#[extendr]
pub fn get_username() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_username();
}

/// return a job descriptor
/// @export
/// @examples get_job_descriptor()
#[extendr]
pub fn get_job_descriptor() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_job_descriptor();
}

/// return a job level
/// @export
/// @examples get_job_level()
#[extendr]
pub fn get_job_level() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_job_level();
}

/// return a job title
/// @export
/// @examples get_job_title()
#[extendr]
pub fn get_job_title() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_job_title();
}

/// return a language abbreviation
/// @export
/// @examples get_language_abbreviation()
#[extendr]
pub fn get_language_abbreviation() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_language_abbreviation();
}

/// return a language programming
/// @export
/// @examples get_language_programming()
#[extendr]
pub fn get_language_programming() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_language_programming();
}

/// return a language programming
/// @export
/// @examples get_language_random()
#[extendr]
pub fn get_language_random() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_language_random();
}

/// return a log level appache
/// @export
/// @examples get_log_level_appache()
#[extendr]
pub fn get_log_level_appache() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_log_level_appache();
}

/// return a log level general
/// @export
/// @examples get_log_level_general()
#[extendr]
pub fn get_log_level_general() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_log_level_general();
}

/// return a log sys log
/// @export
/// @examples get_log_level_sys_log()
#[extendr]
pub fn get_log_level_sys_log() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_log_level_sys_log();
}

/// return a full name
/// @export
/// @examples get_full_name()
#[extendr]
pub fn get_full_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_full_name();
}

/// return a first name
/// @export
/// @examples get_first_name()
#[extendr]
pub fn get_first_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_first_name();
}

/// return a last name
/// @export
/// @examples get_last_name()
#[extendr]
pub fn get_last_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_last_name();
}

/// return a prefix name
/// @export
/// @examples get_prefix_name()
#[extendr]
pub fn get_prefix_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_prefix_name();
}

/// return a suffix name
/// @export
/// @examples get_suffix_name()
#[extendr]
pub fn get_suffix_name() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_suffix_name();
}

/// return a fake password
/// @export
/// @examples get_generate_password(TRUE,TRUE,TRUE,8)
#[extendr]
pub fn get_generate_password(upper: bool, numeric: bool, special: bool, num: i8) -> String {
    return rust_miscellaneous_pkg::faker::faker::get_generate_password(
        upper, numeric, special, num,
    );
}

/// return a credit card type
/// @export
/// @examples get_credit_card_type()
#[extendr]
pub fn get_credit_card_type() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_credit_card_type();
}

/// return a credit card number
/// @export
/// @examples get_credit_card_number()
#[extendr]
pub fn get_credit_card_number() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_credit_card_number();
}

/// return a credit card luhn number
/// @export
/// @examples get_credit_card_luhn_number()
#[extendr]
pub fn get_credit_card_luhn_number() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_credit_card_luhn_number();
}

/// return a credit card luhn number
/// @export
/// @examples get_credit_card_exp()
#[extendr]
pub fn get_credit_card_exp() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_credit_card_exp();
}

/// return a credit card luhn number
/// @export
/// @examples get_credit_card_ccv()
#[extendr]
pub fn get_credit_card_ccv() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_credit_card_ccv();
}

/// return a status code
/// @export
/// @examples get_status_codes_general()
#[extendr]
pub fn get_status_codes_general() -> i16 {
    return rust_miscellaneous_pkg::faker::faker::get_status_codes_general();
}

/// return a status code simple
/// @export
/// @examples get_status_codes_simple()
#[extendr]
pub fn get_status_codes_simple() -> i16 {
    return rust_miscellaneous_pkg::faker::faker::get_status_codes_simple();
}

/// return a uuid v1
/// @export
/// @examples get_uuid_v1()
#[extendr]
pub fn get_uuid_v1() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_uuid_v1();
}

/// return a uuid v4
/// @export
/// @examples get_uuid_v4()
#[extendr]
pub fn get_uuid_v4() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_uuid_v4();
}

/// return a user agent chrome
/// @export
/// @examples get_user_agent_chrome()
#[extendr]
pub fn get_user_agent_chrome() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_chrome();
}

/// return a user agent firefox
/// @export
/// @examples get_user_agent_firefox()
#[extendr]
pub fn get_user_agent_firefox() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_firefox();
}

/// return a user agent platform token
/// @export
/// @examples get_user_agent_linux_platform_token()
#[extendr]
pub fn get_user_agent_linux_platform_token() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_linux_platform_token();
}

/// return a user agent mac platform token
/// @export
/// @examples get_user_agent_mac_platform_token()
#[extendr]
pub fn get_user_agent_mac_platform_token() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_mac_platform_token();
}

/// return a get user agent opera
/// @export
/// @examples get_user_agent_opera()
#[extendr]
pub fn get_user_agent_opera() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_opera();
}

/// return a get user random platform
/// @export
/// @examples get_user_agent_random_platform()
#[extendr]
pub fn get_user_agent_random_platform() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_random_platform();
}

/// return a get user agent safari
/// @export
/// @examples get_user_agent_safari()
#[extendr]
pub fn get_user_agent_safari() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_safari();
}

/// return a get user windows platform
/// @export
/// @examples get_user_agent_windows_platform_token()
#[extendr]
pub fn get_user_agent_windows_platform_token() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_user_agent_windows_platform_token();
}

/// return a car maker
/// @export
/// @examples get_car_maker()
#[extendr]
pub fn get_car_maker() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_car_maker();
}

/// return a car model
/// @export
/// @examples get_car_model()
#[extendr]
pub fn get_car_model() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_car_model();
}

/// return a car fuel
/// @export
/// @examples get_fuel()
#[extendr]
pub fn get_fuel() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_fuel();
}

/// return a transmission gear
/// @export
/// @examples get_transmission_gear()
#[extendr]
pub fn get_transmission_gear() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_transmission_gear();
}

/// return a vehicle type
/// @export
/// @examples get_vehicle_type()
#[extendr]
pub fn get_vehicle_type() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_vehicle_type();
}

/// return a paragraph
/// @export
/// @examples get_paragraph(10,20,10,",")
#[extendr]
pub fn get_paragraph(
    count: i64,
    sentence_count: i64,
    word_count: i64,
    separator: String,
) -> String {
    return rust_miscellaneous_pkg::faker::faker::get_paragraph(
        count,
        sentence_count,
        word_count,
        separator,
    );
}

/// return a question
/// @export
/// @examples get_question()
#[extendr]
pub fn get_question() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_question();
}

/// return a quote
/// @export
/// @examples get_quote()
#[extendr]
pub fn get_quote() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_quote();
}

/// return a quote
/// @export
/// @examples get_sentence(10)
#[extendr]
pub fn get_sentence(word_count: i64) -> String {
    return rust_miscellaneous_pkg::faker::faker::get_sentence(word_count);
}

/// return a quote
/// @export
/// @examples get_word()
#[extendr]
pub fn get_word() -> String {
    return rust_miscellaneous_pkg::faker::faker::get_word();
}

#[extendr]
/// return a HTTP Response
/// @export
/// @examples send_custom_http_request('https://httpbin.org/get','GET',list(),list(),list(),"")
/// send_custom_http_request('https://httpbin.org/post','POST',list(),list(),list(),"")
pub fn send_custom_http_request(
    url: String,
    method: String,
    request_headers: List,
    request_query_params: List,
    request_form: List,
    request_body: String,
) -> String {
    let run_time = tokio::runtime::Runtime::new().unwrap();
    let res = run_time.block_on(
        rust_miscellaneous_pkg::custom_request::custom_request::create_request(
            url,
            method,
            Some(helpers::helpers::convert_robj_hashmap_string_hashmap(
                request_headers.into_hashmap(),
            )),
            Some(helpers::helpers::convert_robj_hashmap_string_hashmap(
                request_query_params.into_hashmap(),
            )),
            Some(helpers::helpers::convert_robj_hashmap_string_hashmap(
                request_form.into_hashmap(),
            )),
            Some(bytes::Bytes::from(request_body)),
        ),
    );

    return serde_json::to_string(&res.unwrap()).unwrap();
}

#[extendr]
/// Resize an image
/// @export
/// @description The filter option should be Nearest, Triangle, CatmullRom, Gaussian, Lanczos3
/// The default option for the image Lanczos3
/// @export
/// @examples resize_image('20220824_214140.jpg',500,500,'Nearest','20220824_214140_nearest.jpg',TRUE)
/// resize_image('20220824_214212.jpg',500,500,'Triangle','20220824_214212_triangle.jpg',TRUE)
/// resize_image('20220824_214215.jpg',500,500,'CatmullRom','20220824_214215_catmullRom.jpg',TRUE)
/// resize_image('20220824_214219.jpg',500,500,'Gaussian','20220824_214219_gaussian.jpg',TRUE)
/// resize_image('20220824_214217.jpg',500,500,'Lanczos3','20220824_214217_lanczos3.jpg',TRUE)
fn resize_image(
    path: String,
    nwidth: u32,
    nheight: u32,
    filter: String,
    path_new_image: String,
    only_shrink: bool,
) -> bool {
    
    rust_miscellaneous_pkg::image_resize::image_resizer::resize(
        path,
        nwidth,
        nheight,
        helpers::helpers::convert_string_to_image_filter(filter),
        path_new_image,
        only_shrink
    )
}

#[extendr]
/// write json to excel
/// @description This is a function to write json arrays or json objects to an excel sheet
/// @export
/// @examples
/// write_sheet_by_name('example.xlsx',"Test1",jsonlite::toJSON(mtcars))
fn write_sheet_by_name(
    file_path: String,
    sheet_name: String,
    json_input_str: String
)-> bool {
    let json_input = serde_json::from_str(&json_input_str);
    let json_input = match json_input{
        Ok(res) => res,
        Err(err) => panic!("{}", err)
    };
    rust_miscellaneous_pkg::excel_operations::excel_operations::write_sheet_by_name(
        file_path,
        sheet_name,
        json_input
    )
}

#[extendr]
/// read excel sheet by name to json
/// @description This is a function to read out the content out of a excel sheet to json array
/// @export
/// @examples
/// read_sheet_by_name("example_input.xlsx","Test1")
fn read_sheet_by_name(file_path: String, sheet_name: String)->String{
    match serde_json::to_string(
        &rust_miscellaneous_pkg::excel_operations::excel_operations::read_sheet_by_name(
            file_path,
            sheet_name
        )
    ){
        Ok(res) => return res,
        Err(err) => panic!("{}", err)
    }
}
#[extendr]
/// convert an xml string to json
/// @description This function converts an xml string to json
/// @export
/// @examples
/// inputStr <- "<dataset>
///                    <record>
///                        <id>1</id>
///                         <first_name>Nerita</first_name>
///                          <last_name>Stanney</last_name>
///                          <email>nstanney0@domainmarket.com</email>
///                          <gender>Female</gender>
///                          <ip_address>223.10.217.33</ip_address>
///                     </record>
///        </dataset>"
/// xml_to_json(inputStr)
/// 
fn xml_to_json(xml_input_str: String)->String{
    match serde_json::to_string(
        &rust_miscellaneous_pkg::xml_to_json_convert::xml_to_json_convert::xml_to_json(xml_input_str)
    ){
        Ok(res) => return res,
        Err(err) => panic!("{}", err)
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod RWrapperMiscellaneousPkg;
    fn hello_world;
    fn read_file_to_string;
    fn write_file_from_char_vec;
    fn hamming;
    fn levenshtein;
    fn levenshtein_smid;
    fn hamming_smid;
    fn bom_matching;
    fn bndm_matching;
    fn horspool_matching;
    fn kmp_matching;
    fn get_sha224;
    fn get_sha256;
    fn get_sha384;
    fn get_sha512;
    fn get_sha1;
    fn get_random_number_range;
    fn get_city;
    fn get_country;
    fn get_country_abr;
    fn get_latitude;
    fn get_longitude;
    fn get_state;
    fn get_street_name;
    fn get_street_number;
    fn get_street_prefix;
    fn get_street_suffix;
    fn get_zip;
    fn get_anmimal;
    fn get_cat;
    fn get_dog;
    fn get_farm;
    fn get_pet_name;
    fn get_alcohol;
    fn get_blg;
    fn get_hop;
    fn get_ibu;
    fn get_malt;
    fn get_name;
    fn get_style;
    fn get_yeast;
    fn get_bool;
    fn get_full_color;
    fn get_hex_color;
    fn get_safe_color;
    fn get_company;
    fn get_buzzword;
    fn get_bs;
    fn get_company_suffix;
    fn get_email;
    fn get_phone;
    fn get_phone_formatted;
    fn get_currency_long;
    fn get_currency_short;
    fn get_day;
    fn get_hour;
    fn get_minute;
    fn get_secound;
    fn get_timezone;
    fn get_week_day;
    fn get_year;
    fn get_extension;
    fn get_mime_type;
    fn get_abbreviation;
    fn get_adjective;
    fn get_ingverb;
    fn add_num;
    fn sub_num;
    fn mul_num;
    fn div_num;
    fn mod_num;
    fn get_street;
    fn get_noun;
    fn get_pharse;
    fn get_verb;
    fn get_hippster_word;
    fn get_domain_name;
    fn get_domain_suffix;
    fn get_http_method;
    fn get_ipv4_address;
    fn get_ipv6_address;
    fn get_mac_address;
    fn get_username;
    fn get_job_descriptor;
    fn get_job_level;
    fn get_job_title;
    fn get_language_abbreviation;
    fn get_language_programming;
    fn get_language_random;
    fn get_log_level_appache;
    fn get_log_level_general;
    fn get_log_level_sys_log;
    fn get_full_name;
    fn get_first_name;
    fn get_last_name;
    fn get_prefix_name;
    fn get_suffix_name;
    fn get_generate_password;
    fn get_credit_card_type;
    fn get_credit_card_number;
    fn get_credit_card_luhn_number;
    fn get_credit_card_exp;
    fn get_credit_card_ccv;
    fn get_status_codes_general;
    fn get_status_codes_simple;
    fn get_uuid_v1;
    fn get_uuid_v4;
    fn get_user_agent_chrome;
    fn get_user_agent_firefox;
    fn get_user_agent_linux_platform_token;
    fn get_user_agent_mac_platform_token;
    fn get_user_agent_opera;
    fn get_user_agent_random_platform;
    fn get_user_agent_safari;
    fn get_user_agent_windows_platform_token;
    fn get_car_maker;
    fn get_car_model;
    fn get_fuel;
    fn get_transmission_gear;
    fn get_vehicle_type;
    fn get_paragraph;
    fn get_question;
    fn get_quote;
    fn get_sentence;
    fn get_word;
    fn send_custom_http_request;
    fn resize_image;
    fn write_sheet_by_name;
    fn read_sheet_by_name;
    fn xml_to_json;
}
