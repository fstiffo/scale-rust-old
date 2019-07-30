#![warn(clippy::all)]

use chrono::{NaiveDate, NaiveDateTime};
use decimal::d128;
use kairos::timetype::TimeType as TT;

pub enum Condomino {
    Michela,
    Gerardo,
    Elena,
    Giulia,
}

pub enum Operazione {
    VersamentoQuote(Condomino, d128),
    PagamentoScale,
    AltraSpesa(String, d128),
    AltroVersamento(String, d128),
    Prestito(d128),
    Restituzione(d128),
}

pub type Movimento = (TT, Operazione);

pub struct Param {
    costo_scale: d128,
    num_pulize_mese: u32,
    quota_mensile: d128,
}

pub type Attuale = (TT, Param);

use Condomino as C;

const CONDOMINI: [Condomino; 4] = [C::Michela, C::Gerardo, C::Elena, C::Giulia];

macro_rules! moment {
    ($y:expr, $m:expr, $d:expr) => {
        TT::Moment(<NaiveDate>::from_ymd($y, $m, $d).and_hms(0, 0, 0))
    };
}

const ANNO_ZERO: i32 = 2019;
const MESE_ZERO: u32 = 7;
const GIORNO_ZERO: u32 = 1;

macro_rules! tempo_zero {
    () => {
        TT::Moment(<NaiveDate>::from_ymd(ANNO_ZERO, MESE_ZERO, GIORNO_ZERO).and_hms(0, 0, 0))
    };
}

pub fn setup_attuale() -> Attuale {
    (
        tempo_zero!(),
        Param {
            costo_scale: d128!(20),
            num_pulize_mese: 2,
            quota_mensile: d128!(12),
        },
    )
}

use Operazione as Op;

pub fn setupMovimenti() -> Vec<Movimento> {
    vec![
        (
            tempo_zero!(),
            Op::AltroVersamento("Appianamento".to_string(), d128!(333)),
        ),
        (tempo_zero!(), Op::VersamentoQuote(C::Michela, d128!(74))),
        (tempo_zero!(), Op::VersamentoQuote(C::Gerardo, d128!(78))),
        (tempo_zero!(), Op::VersamentoQuote(C::Elena, d128!(48))),
        (moment!(2019, 7, 22), Op::Prestito(d128!(500))),
        (moment!(2019, 7, 11), Op::PagamentoScale),
    ]
}
