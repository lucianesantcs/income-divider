slint::include_modules!();

const TAX_PER: f64 = 0.30;
const OWNER_PER: f64 = 0.55;
const PROFIT_PER: f64 = 0.05;
const OPEX_PER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_divide_income({
        move |string| {
            let ui = ui_handle.unwrap();
            let parsed_string: f64 = string.trim().parse().unwrap();

            let tax: f64 = parsed_string * TAX_PER;
            let owner: f64 = parsed_string * OWNER_PER;
            let profit: f64 = parsed_string * PROFIT_PER;
            let opex: f64 = parsed_string * OPEX_PER;

            let result = format!(
                "Taxes: {:.2} \nOwner: {:.2} \nProfit: {:.2} \nOpex: {:.2}",
                tax, owner, profit, opex
            );

            ui.set_results(result.into())
        }
    });

    ui.run()
}
