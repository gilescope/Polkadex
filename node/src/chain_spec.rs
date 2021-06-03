use frame_benchmarking::frame_support::PalletId;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{AccountIdConversion, IdentifyAccount, Verify};

use thea_node_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
    SystemConfig, TheaConfig, WASM_BINARY,
};
use thea_primitives::ecdsa::AuthorityId as TheaId;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId, TheaId) {
    (
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<TheaId>(s),
    )
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Polkadex Testnet",
        // ID
        "polkadex_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    // 100 keys for testnet
                    authority_keys_from_seed("rabbit when hundred because during expect miracle illegal make alarm grunt crunch"),
                    authority_keys_from_seed("total outside zebra result dune miracle double casual vendor odor gather arm"),
                    authority_keys_from_seed("universe possible flag floor viable type just cover hedgehog stick cloud ghost"),
                    authority_keys_from_seed("casual inmate shrug fragile render student sweet outside debris lounge dizzy surge"),
                    authority_keys_from_seed("trick science ugly foster virtual forward valley then fantasy smoke host mutual"),
                    authority_keys_from_seed("obey shrug spoon hurry border can video service salmon hope virus tilt"),
                    authority_keys_from_seed("purchase wrestle gown sustain arch ahead merry afraid rib replace wealth mind"),
                    authority_keys_from_seed("display labor arrow song science favorite produce later board weekend extend code"),
                    authority_keys_from_seed("replace when act insect gorilla angle tackle swarm fluid perfect art despair"),
                    authority_keys_from_seed("cycle alarm dad image cinnamon help table antique topic rail ticket wave"),
                    authority_keys_from_seed("improve iron grunt planet maximum sorry exotic upon series movie uncle sword"),
                    authority_keys_from_seed("maximum milk weather health since feel visual mother shop mouse various enrich"),
                    authority_keys_from_seed("maid fame evidence kit token canvas buzz display mixture tool couch success"),
                    authority_keys_from_seed("level popular pipe crater topic air cheap gold return fashion swamp attitude"),
                    authority_keys_from_seed("mimic pass weasel sausage crawl ice sunset scheme into fuel hospital rug"),
                    authority_keys_from_seed("artist enough flight advice truth history present post fine oval thumb tuition"),
                    authority_keys_from_seed("cost excite load april journey brick famous power toss engine impulse unhappy"),
                    authority_keys_from_seed("west radar session lamp wise relax glance divert proud vital toilet worth"),
                    authority_keys_from_seed("mom spawn crush sick eternal sweet pigeon fish stairs link essence couch"),
                    authority_keys_from_seed("scheme review fortune carpet labor duty junior milk differ scrap swing turn"),
                    authority_keys_from_seed("chronic glide alien process prosper case wagon such siren grape juice recycle"),
                    authority_keys_from_seed("chronic egg fire canvas follow bachelor violin armor weekend that circle own"),
                    authority_keys_from_seed("tail unusual peace define action occur crater pear cycle wash swift immune"),
                    authority_keys_from_seed("occur physical hold slim squirrel legend enroll uphold size category arrange better"),
                    authority_keys_from_seed("dance saddle patch believe visual raven garment cluster main top police worry"),
                    authority_keys_from_seed("trophy proud visit select father addict jaguar skin order lion report phrase"),
                    authority_keys_from_seed("bleak victory opera perfect blast stay foam dad hunt proof squeeze win"),
                    authority_keys_from_seed("magnet brass say light craft sick zero else can more slender detail"),
                    authority_keys_from_seed("satoshi tiny rail regular scrap paper grief pride accuse lens company deputy"),
                    authority_keys_from_seed("story loan talent trial safe juice jazz cliff receive push volcano reduce"),
                    authority_keys_from_seed("coconut misery upset aware heavy cable hello coyote chicken horn black quote"),
                    authority_keys_from_seed("crane other material chicken fringe hurry latin garbage retire swear snap never"),
                    authority_keys_from_seed("history youth obvious supply athlete stage salute year fetch satisfy actor consider"),
                    authority_keys_from_seed("remain twist april gossip bid neither frost bird super inside across portion"),
                    authority_keys_from_seed("close switch maze smart seat hover cloth twelve home flavor salad badge"),
                    authority_keys_from_seed("window tip cereal great top trouble session clinic service cloth vast into"),
                    authority_keys_from_seed("entire aerobic tired grunt pass where pattern pond swallow spoil kite orphan"),
                    authority_keys_from_seed("armed suit push sleep cook lumber night destroy salmon ozone trade unlock"),
                    authority_keys_from_seed("wear fuel robot alpha brick fatal clean swallow mom dry weather sight"),
                    authority_keys_from_seed("noise scale captain cupboard blind depart welcome retreat machine arch rain glad"),
                    authority_keys_from_seed("shaft one scissors scale juice clay zoo brass cheese nasty female put"),
                    authority_keys_from_seed("bonus organ glove kick sorry cigar bottom page swap shy shallow gentle"),
                    authority_keys_from_seed("stem coil library pilot reflect program very vacuum swallow mule force tortoise"),
                    authority_keys_from_seed("smart diary nuclear ride estate fox pluck include work alone tourist cliff"),
                    authority_keys_from_seed("science glue abstract rally horse term island short often floor spirit enough"),
                    authority_keys_from_seed("safe hard drum robot monkey initial neither deputy tide army crop shove"),
                    authority_keys_from_seed("number buddy weapon outer journey scorpion brush identify raw athlete sadness person"),
                    authority_keys_from_seed("argue oval frog rug weather margin marine panda slogan place scatter circle"),
                    authority_keys_from_seed("wing render uncle elevator great edge wash about art later spell list"),
                    authority_keys_from_seed("glide mercy air meat alley wink base crumble trend air ten laundry"),
                    authority_keys_from_seed("iron jump duty supreme club west cradle future organ era clever twin"),
                    authority_keys_from_seed("jeans alien heavy cash artwork walnut absorb special session sample unable venture"),
                    authority_keys_from_seed("final wolf floor mean brisk ketchup muscle little snow will parrot setup"),
                    authority_keys_from_seed("solid volcano sing frequent decorate dinner banner arrive surge plunge wheel sick"),
                    authority_keys_from_seed("notable speak oblige exit dynamic entry pink trap olympic flame grab chef"),
                    authority_keys_from_seed("planet hundred fluid bracket weird knife inmate myth stumble path summer buyer"),
                    authority_keys_from_seed("swim night deal smile exile base level argue picture rude mango art"),
                    authority_keys_from_seed("lava guess spare swamp camera riot loyal honey comic brick miracle focus"),
                    authority_keys_from_seed("shed original student morning pair fork steel spot chief faculty age tattoo"),
                    authority_keys_from_seed("vivid desk seminar shop knee negative oil dilemma gift into maid force"),
                    authority_keys_from_seed("bracket situate scale vintage happy race rookie found fresh depart argue small"),
                    authority_keys_from_seed("enlist traffic tourist affair timber gate famous rack abandon habit tool liberty"),
                    authority_keys_from_seed("correct picture bless exotic risk echo tank worry sad exist unfold motor"),
                    authority_keys_from_seed("thrive huge all perfect vanish chimney artwork swear zebra glad noble stomach"),
                    authority_keys_from_seed("park orphan dad later canvas horse monkey door iron water odor chief"),
                    authority_keys_from_seed("soldier diamond carpet garlic reflect attitude lamp initial license soda music gorilla"),
                    authority_keys_from_seed("dutch daring attract eye memory scissors swear snow cable belt cake crush"),
                    authority_keys_from_seed("visual unique chaos october skin tobacco luggage spoil any tobacco can episode"),
                    authority_keys_from_seed("boil furnace merge range balance round loop sort use fade ring increase"),
                    authority_keys_from_seed("employ able among color office ozone brown fiscal hub notice theory sudden"),
                    authority_keys_from_seed("coyote abandon height shell toilet sauce grid wrap melody young trash current"),
                    authority_keys_from_seed("beef light shaft purse knock blur better indoor ten prevent shield battle"),
                    authority_keys_from_seed("travel bag crack state sleep middle decorate voyage case neutral upgrade knee"),
                    authority_keys_from_seed("age educate field winner carpet stage music share buyer salt jealous flash"),
                    authority_keys_from_seed("hover adjust champion iron response rifle can nurse festival sphere comfort frame"),
                    authority_keys_from_seed("panda surface fog utility beauty lady differ net mother vicious olive pottery"),
                    authority_keys_from_seed("volcano supply gorilla enemy tunnel gas chaos clever eternal agent mother ignore"),
                    authority_keys_from_seed("topple eyebrow top soup obey pill decide correct lava issue flip maze"),
                    authority_keys_from_seed("craft ready escape seminar promote hawk argue cotton keen kit laugh oyster"),
                    authority_keys_from_seed("bunker benefit elder book pudding position what dry friend unfold man waste"),
                    authority_keys_from_seed("episode alarm catalog decrease limit used electric sting arrive cherry sketch reform"),
                    authority_keys_from_seed("grid mechanic noise special cement tube injury patch detail comic casino river"),
                    authority_keys_from_seed("crime cousin shoulder thank corn disorder hotel unknown clerk master trigger broom"),
                    authority_keys_from_seed("add labor feed coral boy hungry night blanket major alpha december feed"),
                    authority_keys_from_seed("noise harvest then entry company ticket valve stool zero piano proof nothing"),
                    authority_keys_from_seed("pond bus erosion phrase basket require outer vibrant squirrel action hip custom"),
                    authority_keys_from_seed("exotic enough benefit coral avocado issue hawk airport radio panda tree cave"),
                    authority_keys_from_seed("scale people dinner garbage math grid police ranch scrub explain room connect"),
                    authority_keys_from_seed("symptom veteran vintage phrase hungry join surprise naive all animal work soccer"),
                    authority_keys_from_seed("vapor actress retreat gentle lemon castle bean exhaust gather torch basket remind"),
                    authority_keys_from_seed("shed jar use head mutual wash gauge approve still attend focus dial"),
                    authority_keys_from_seed("aim sheriff option spray adjust poverty crater ankle slab million patient sunset"),
                    authority_keys_from_seed("pole phone orbit salute large vocal reopen mean vicious pear ready design"),
                    authority_keys_from_seed("orbit tumble harbor risk garden analyst tape life fat cable hazard unlock"),
                    authority_keys_from_seed("pipe horn luxury galaxy monkey pioneer fever urge tone visual suspect ivory"),
                    authority_keys_from_seed("recycle top mushroom melt bid citizen olympic unhappy cage sketch polar trash"),
                    authority_keys_from_seed("apple penalty will drip degree sauce bid follow vast glare awkward hamster"),
                    authority_keys_from_seed("cattle another version state emotion erode kiwi joke mercy hire journey drastic"),
                    authority_keys_from_seed("vintage about wing liberty student hover sunset where between tornado tongue goat"),
                    authority_keys_from_seed("fish topic entry retire just virtual direct scorpion please immense pink enact"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub const OCEXGenesisAccount: PalletId = PalletId(*b"polka/ga");

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId, TheaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    //let genesis = get_account_id_from_seed::<sr25519::Public>("Gen");
    let genesis: AccountId = OCEXGenesisAccount.into_account();
    GenesisConfig {
        frame_system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        pallet_aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        pallet_grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        pallet_sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key.clone(),
        },
        thea_pallet: TheaConfig {
            authorities: initial_authorities.iter().map(|x| (x.2.clone())).collect(),
            can_start: false,
        },
    }
}
