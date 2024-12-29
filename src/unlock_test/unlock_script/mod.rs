use crate::unlock_test::Service;

pub mod animefesta;
pub mod bahamut;
pub mod bbc_iplayer;
pub mod bilibili;
pub mod dazn;
pub mod four_gtv;
pub mod google_play_store;
pub mod hami_video;
pub mod hbomax;
pub mod hulu_jp;
pub mod iqiyi_oversea;
pub mod kancolle;
pub mod lemino;
pub mod mora;
pub mod mytv_super;
pub mod netflix;
pub mod nowe;
pub mod princess_connect_redive_japan;
pub mod showmax;
pub mod sling_tv;
pub mod steam;
pub mod unext;
pub mod viutv;
pub mod youtube_cdn;
pub mod youtube_premium;

pub fn all_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(netflix::Netflix),                   // Global
        Box::new(hbomax::HboMax),                     // Global
        Box::new(youtube_cdn::YoutubeCDN),            // Global
        Box::new(youtube_premium::YoutubePremium),    // Global
        Box::new(google_play_store::GooglePlayStore), // Global
        Box::new(iqiyi_oversea::IqiyiOversea),        // Asia, US
        Box::new(steam::Steam),                       // Global
        Box::new(bahamut::BahamutAnime),              // HK, TW, MO
        Box::new(bilibili::BilibiliChinaMainland),    // China
        Box::new(bilibili::BilibiliChinaTWOnly),      // TW
        Box::new(bilibili::BilibiliChinaHKMOTW),      // HK, TW, MO
        Box::new(princess_connect_redive_japan::PrincessConnectReDiveJapan), // JP
        Box::new(kancolle::Kancolle),                 // JP
        Box::new(lemino::Lemino),                     // JP
        Box::new(animefesta::AnimeFesta),             // JP
        Box::new(mora::Mora),                         // JP
        Box::new(bbc_iplayer::BBCIPlayer),            // UK, Euro
        Box::new(dazn::Dazn),                         // Global
        Box::new(hulu_jp::HuluJP),                    // JP
        Box::new(mytv_super::MyTVSuper),              // HK, MO
        Box::new(nowe::NowE),                         // HK
        Box::new(viutv::ViuTV),                       // HK
        Box::new(unext::UNext),                       // JP
        Box::new(four_gtv::FourGTV),                  // TW
        Box::new(sling_tv::SlingTV),                  // US
        Box::new(hami_video::HamiVideo),              // TW
        Box::new(showmax::ShowMax),                   // Afr
    ]
}

pub fn global_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(netflix::Netflix),                   // Global
        Box::new(hbomax::HboMax),                     // Global
        Box::new(youtube_cdn::YoutubeCDN),            // Global
        Box::new(youtube_premium::YoutubePremium),    // Global
        Box::new(google_play_store::GooglePlayStore), // Global
        Box::new(steam::Steam),                       // Global
    ]
}

pub fn hk_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(bahamut::BahamutAnime),         // HK, TW, MO
        Box::new(bilibili::BilibiliChinaHKMOTW), // HK, TW, MO
        Box::new(mytv_super::MyTVSuper),         // HK, MO
        Box::new(nowe::NowE),                    // HK
        Box::new(viutv::ViuTV),                  // HK
    ]
}

pub fn tw_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(bahamut::BahamutAnime),         // HK, TW, MO
        Box::new(bilibili::BilibiliChinaTWOnly), // TW
        Box::new(bilibili::BilibiliChinaHKMOTW), // HK, TW, MO
        Box::new(four_gtv::FourGTV),             // TW
        Box::new(hami_video::HamiVideo),         // TW
    ]
}

pub fn mo_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(bahamut::BahamutAnime),         // HK, TW, MO
        Box::new(bilibili::BilibiliChinaHKMOTW), // HK, TW, MO
        Box::new(mora::Mora),                    // JP
        Box::new(mytv_super::MyTVSuper),         // HK, MO
    ]
}

pub fn cn_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(bilibili::BilibiliChinaMainland), // China
        Box::new(iqiyi_oversea::IqiyiOversea),     // Asia, US
    ]
}

pub fn asia_services() -> Vec<Box<dyn Service + Send + Sync>> {
    let mut service = Vec::new();
    service.extend(jp_services());
    service.extend(hk_services());
    service.extend(mo_services());
    service.extend(tw_services());
    service.extend(cn_services());
    service
}

pub fn jp_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(princess_connect_redive_japan::PrincessConnectReDiveJapan),
        Box::new(kancolle::Kancolle),
        Box::new(lemino::Lemino),
        Box::new(animefesta::AnimeFesta),
        Box::new(mora::Mora),
        Box::new(hulu_jp::HuluJP),
        Box::new(unext::UNext),
    ]
}

pub fn afr_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![Box::new(showmax::ShowMax)]
}

pub fn uk_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![Box::new(bbc_iplayer::BBCIPlayer)]
}

pub fn us_services() -> Vec<Box<dyn Service + Send + Sync>> {
    vec![
        Box::new(sling_tv::SlingTV),
        Box::new(iqiyi_oversea::IqiyiOversea), // Asia, US
    ]
}

pub fn euro_services() -> Vec<Box<dyn Service + Send + Sync>> {
    let mut service = Vec::new();
    service.extend(uk_services());
    service
}
