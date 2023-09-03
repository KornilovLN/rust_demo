//-----------------------------------------------------------------------------
//--- RUST-DEMO 
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   03.09.2023 13:10:00
//-----------------------------------------------------------------------------
//--- Все сводится к "Привет, Struct!" и испытанию dorrow и меток времени жизни
//-----------------------------------------------------------------------------

//--- для выхода из программы с ненулевым сообщением системе
use std::process;

fn main() {

    //--- Основная работа ПО
	if let Err(e) = rust_demo::run() {
		eprintln!("Main: Ошибка в приложении: {}", e);		
		process::exit(1);
	}   
}