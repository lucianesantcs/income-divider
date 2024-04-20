slint::include_modules!();
use currency_rs::Currency;
use currency_rs::CurrencyOpts;

const CASA: f64 = 0.55;
const INVESTIMENTO: f64 = 0.30;
const LAZER: f64 = 0.10;
const EMERGENCIA: f64 = 0.05;

fn format_currency(value: f64) -> String {
    let otp = CurrencyOpts::new()
        .set_separator(".")
        .set_decimal(",")
        .set_symbol("R$ ");

    Currency::new_float(value, Some(otp)).format()
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income({
        move |string| {
            let ui = ui_handle.unwrap();
            let parsed_string: f64 = string.trim().parse().unwrap();

            let casa = format_currency(parsed_string * CASA);
            let investimento = format_currency(parsed_string * INVESTIMENTO);
            let lazer = format_currency(parsed_string * LAZER);
            let emergencia = format_currency(parsed_string * EMERGENCIA);

            let result = format!(
                "Casa: {} \nInvestimento: {} \nLazer: {} \nEmergencia: {}",
                casa, investimento, lazer, emergencia
            );

            ui.set_results(result.into())
        }
    });

    ui.run()
}
