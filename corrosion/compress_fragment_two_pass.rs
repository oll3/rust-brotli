extern {
    fn BrotliBuildAndStoreHuffmanTreeFast(
        m : *mut MemoryManager,
        histogram : *const u32,
        histogram_total : usize,
        max_bits : usize,
        depth : *mut u8,
        bits : *mut u16,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn BrotliConvertBitDepthsToSymbols(
        depth : *const u8, len : usize, bits : *mut u16
    );
    fn BrotliCreateHuffmanTree(
        data : *const u32,
        length : usize,
        tree_limit : i32,
        tree : *mut HuffmanTree,
        depth : *mut u8
    );
    fn BrotliStoreHuffmanTree(
        depths : *const u8,
        num : usize,
        tree : *mut HuffmanTree,
        storage_ix : *mut usize,
        storage : *mut u8
    );
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn log2(__x : f64) -> f64;
    fn memcmp(
        __s1 : *const ::std::os::raw::c_void,
        __s2 : *const ::std::os::raw::c_void,
        __n : usize
    ) -> i32;
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

static kCompressFragmentTwoPassBlockSize
    : usize
    = (1i32 << 17i32) as (usize);

static mut kLog2Table
    : [f32; 256]
    = [   0.0000000000000000f32,
          0.0000000000000000f32,
          1.0000000000000000f32,
          1.5849625007211563f32,
          2.0000000000000000f32,
          2.3219280948873622f32,
          2.5849625007211561f32,
          2.8073549220576042f32,
          3.0000000000000000f32,
          3.1699250014423126f32,
          3.3219280948873626f32,
          3.4594316186372978f32,
          3.5849625007211565f32,
          3.7004397181410922f32,
          3.8073549220576037f32,
          3.9068905956085187f32,
          4.0000000000000000f32,
          4.0874628412503400f32,
          4.1699250014423122f32,
          4.2479275134435852f32,
          4.3219280948873626f32,
          4.3923174227787607f32,
          4.4594316186372973f32,
          4.5235619560570131f32,
          4.5849625007211570f32,
          4.6438561897747244f32,
          4.7004397181410926f32,
          4.7548875021634691f32,
          4.8073549220576037f32,
          4.8579809951275728f32,
          4.9068905956085187f32,
          4.9541963103868758f32,
          5.0000000000000000f32,
          5.0443941193584534f32,
          5.0874628412503400f32,
          5.1292830169449664f32,
          5.1699250014423122f32,
          5.2094533656289501f32,
          5.2479275134435852f32,
          5.2854022188622487f32,
          5.3219280948873626f32,
          5.3575520046180838f32,
          5.3923174227787607f32,
          5.4262647547020979f32,
          5.4594316186372973f32,
          5.4918530963296748f32,
          5.5235619560570131f32,
          5.5545888516776376f32,
          5.5849625007211570f32,
          5.6147098441152083f32,
          5.6438561897747244f32,
          5.6724253419714961f32,
          5.7004397181410926f32,
          5.7279204545631996f32,
          5.7548875021634691f32,
          5.7813597135246599f32,
          5.8073549220576046f32,
          5.8328900141647422f32,
          5.8579809951275719f32,
          5.8826430493618416f32,
          5.9068905956085187f32,
          5.9307373375628867f32,
          5.9541963103868758f32,
          5.9772799234999168f32,
          6.0000000000000000f32,
          6.0223678130284544f32,
          6.0443941193584534f32,
          6.0660891904577721f32,
          6.0874628412503400f32,
          6.1085244567781700f32,
          6.1292830169449672f32,
          6.1497471195046822f32,
          6.1699250014423122f32,
          6.1898245588800176f32,
          6.2094533656289510f32,
          6.2288186904958804f32,
          6.2479275134435861f32,
          6.2667865406949019f32,
          6.2854022188622487f32,
          6.3037807481771031f32,
          6.3219280948873617f32,
          6.3398500028846252f32,
          6.3575520046180847f32,
          6.3750394313469254f32,
          6.3923174227787598f32,
          6.4093909361377026f32,
          6.4262647547020979f32,
          6.4429434958487288f32,
          6.4594316186372982f32,
          6.4757334309663976f32,
          6.4918530963296748f32,
          6.5077946401986964f32,
          6.5235619560570131f32,
          6.5391588111080319f32,
          6.5545888516776376f32,
          6.5698556083309478f32,
          6.5849625007211561f32,
          6.5999128421871278f32,
          6.6147098441152092f32,
          6.6293566200796095f32,
          6.6438561897747253f32,
          6.6582114827517955f32,
          6.6724253419714952f32,
          6.6865005271832185f32,
          6.7004397181410917f32,
          6.7142455176661224f32,
          6.7279204545631988f32,
          6.7414669864011465f32,
          6.7548875021634691f32,
          6.7681843247769260f32,
          6.7813597135246599f32,
          6.7944158663501062f32,
          6.8073549220576037f32,
          6.8201789624151887f32,
          6.8328900141647422f32,
          6.8454900509443757f32,
          6.8579809951275719f32,
          6.8703647195834048f32,
          6.8826430493618416f32,
          6.8948177633079437f32,
          6.9068905956085187f32,
          6.9188632372745955f32,
          6.9307373375628867f32,
          6.9425145053392399f32,
          6.9541963103868758f32,
          6.9657842846620879f32,
          6.9772799234999168f32,
          6.9886846867721664f32,
          7.0000000000000000f32,
          7.0112272554232540f32,
          7.0223678130284544f32,
          7.0334230015374501f32,
          7.0443941193584534f32,
          7.0552824355011898f32,
          7.0660891904577721f32,
          7.0768155970508317f32,
          7.0874628412503400f32,
          7.0980320829605272f32,
          7.1085244567781700f32,
          7.1189410727235076f32,
          7.1292830169449664f32,
          7.1395513523987937f32,
          7.1497471195046822f32,
          7.1598713367783891f32,
          7.1699250014423130f32,
          7.1799090900149345f32,
          7.1898245588800176f32,
          7.1996723448363644f32,
          7.2094533656289492f32,
          7.2191685204621621f32,
          7.2288186904958804f32,
          7.2384047393250794f32,
          7.2479275134435861f32,
          7.2573878426926521f32,
          7.2667865406949019f32,
          7.2761244052742384f32,
          7.2854022188622487f32,
          7.2946207488916270f32,
          7.3037807481771031f32,
          7.3128829552843557f32,
          7.3219280948873617f32,
          7.3309168781146177f32,
          7.3398500028846243f32,
          7.3487281542310781f32,
          7.3575520046180847f32,
          7.3663222142458151f32,
          7.3750394313469254f32,
          7.3837042924740528f32,
          7.3923174227787607f32,
          7.4008794362821844f32,
          7.4093909361377026f32,
          7.4178525148858991f32,
          7.4262647547020979f32,
          7.4346282276367255f32,
          7.4429434958487288f32,
          7.4512111118323299f32,
          7.4594316186372973f32,
          7.4676055500829976f32,
          7.4757334309663976f32,
          7.4838157772642564f32,
          7.4918530963296748f32,
          7.4998458870832057f32,
          7.5077946401986964f32,
          7.5156998382840436f32,
          7.5235619560570131f32,
          7.5313814605163119f32,
          7.5391588111080319f32,
          7.5468944598876373f32,
          7.5545888516776376f32,
          7.5622424242210728f32,
          7.5698556083309478f32,
          7.5774288280357487f32,
          7.5849625007211561f32,
          7.5924570372680806f32,
          7.5999128421871278f32,
          7.6073303137496113f32,
          7.6147098441152075f32,
          7.6220518194563764f32,
          7.6293566200796095f32,
          7.6366246205436488f32,
          7.6438561897747244f32,
          7.6510516911789290f32,
          7.6582114827517955f32,
          7.6653359171851765f32,
          7.6724253419714952f32,
          7.6794800995054464f32,
          7.6865005271832185f32,
          7.6934869574993252f32,
          7.7004397181410926f32,
          7.7073591320808825f32,
          7.7142455176661224f32,
          7.7210991887071856f32,
          7.7279204545631996f32,
          7.7347096202258392f32,
          7.7414669864011465f32,
          7.7481928495894596f32,
          7.7548875021634691f32,
          7.7615512324444795f32,
          7.7681843247769260f32,
          7.7747870596011737f32,
          7.7813597135246608f32,
          7.7879025593914317f32,
          7.7944158663501062f32,
          7.8008998999203047f32,
          7.8073549220576037f32,
          7.8137811912170374f32,
          7.8201789624151887f32,
          7.8265484872909159f32,
          7.8328900141647422f32,
          7.8392037880969445f32,
          7.8454900509443757f32,
          7.8517490414160571f32,
          7.8579809951275719f32,
          7.8641861446542798f32,
          7.8703647195834048f32,
          7.8765169465650002f32,
          7.8826430493618425f32,
          7.8887432488982601f32,
          7.8948177633079446f32,
          7.9008668079807496f32,
          7.9068905956085187f32,
          7.9128893362299619f32,
          7.9188632372745955f32,
          7.9248125036057813f32,
          7.9307373375628867f32,
          7.9366379390025719f32,
          7.9425145053392399f32,
          7.9483672315846778f32,
          7.9541963103868758f32,
          7.9600019320680806f32,
          7.9657842846620870f32,
          7.9715435539507720f32,
          7.9772799234999168f32,
          7.9829935746943104f32,
          7.9886846867721664f32,
          7.9943534368588578f32
      ];

static mut kInsBase
    : [u32; 24]
    = [   0i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          8i32 as (u32),
          10i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          26i32 as (u32),
          34i32 as (u32),
          50i32 as (u32),
          66i32 as (u32),
          98i32 as (u32),
          130i32 as (u32),
          194i32 as (u32),
          322i32 as (u32),
          578i32 as (u32),
          1090i32 as (u32),
          2114i32 as (u32),
          6210i32 as (u32),
          22594i32 as (u32)
      ];

static mut kInsExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          24i32 as (u32)
      ];

static mut kCopyBase
    : [u32; 24]
    = [   2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          22i32 as (u32),
          30i32 as (u32),
          38i32 as (u32),
          54i32 as (u32),
          70i32 as (u32),
          102i32 as (u32),
          134i32 as (u32),
          198i32 as (u32),
          326i32 as (u32),
          582i32 as (u32),
          1094i32 as (u32),
          2118i32 as (u32)
      ];

static mut kCopyExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          24i32 as (u32)
      ];

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kUTF8ContextLookup
    : [u8; 512]
    = [   0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          8i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          20i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          24i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          32i32 as (u8),
          12i32 as (u8),
          36i32 as (u8),
          12i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          32i32 as (u8),
          32i32 as (u8),
          24i32 as (u8),
          40i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8)
      ];

static mut kSigned3BitContextLookup
    : [u8; 256]
    = [   0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          7i32 as (u8)
      ];

static kHashMul32 : u32 = 0x1e35a7bdi32 as (u32);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
}

unsafe extern fn Log2FloorNonZero(mut n : usize) -> u32 {
    let mut result : u32 = 0i32 as (u32);
    'loop1: loop {
        if {
               n = n >> 1i32;
               n
           } != 0 {
            result = result.wrapping_add(1 as (u32));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    result
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

unsafe extern fn BROTLI_UNALIGNED_LOAD64(
    mut p : *const ::std::os::raw::c_void
) -> usize {
    let mut t : usize;
    memcpy(
        &mut t as (*mut usize) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<usize>()
    );
    t
}

unsafe extern fn Hash(
    mut p : *const u8, mut shift : usize
) -> u32 {
    let h
        : usize
        = (BROTLI_UNALIGNED_LOAD64(
               p as (*const ::std::os::raw::c_void)
           ) << 16i32).wrapping_mul(
              kHashMul32 as (usize)
          );
    (h >> shift) as (u32)
}

unsafe extern fn BROTLI_UNALIGNED_LOAD32(
    mut p : *const ::std::os::raw::c_void
) -> u32 {
    let mut t : u32;
    memcpy(
        &mut t as (*mut u32) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<u32>()
    );
    t
}

unsafe extern fn IsMatch(
    mut p1 : *const u8, mut p2 : *const u8
) -> i32 {
    if !!(BROTLI_UNALIGNED_LOAD32(
              p1 as (*const ::std::os::raw::c_void)
          ) == BROTLI_UNALIGNED_LOAD32(
                   p2 as (*const ::std::os::raw::c_void)
               ) && (*p1.offset(4i32 as (isize)) as (i32) == *p2.offset(
                                                                  4i32 as (isize)
                                                              ) as (i32)) && (*p1.offset(
                                                                                   5i32 as (isize)
                                                                               ) as (i32) == *p2.offset(
                                                                                                  5i32 as (isize)
                                                                                              ) as (i32))) {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn unopt_ctzll(mut val : usize) -> u8 {
    let mut cnt : u8 = 0i32 as (u8);
    'loop1: loop {
        if val & 1i32 as (usize) == 0i32 as (usize) {
            val = val >> 1i32;
            cnt = (cnt as (i32) + 1) as (u8);
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    cnt
}

unsafe extern fn FindMatchLengthWithLimit(
    mut s1 : *const u8, mut s2 : *const u8, mut limit : usize
) -> usize {
    let mut matched : usize = 0i32 as (usize);
    let mut limit2
        : usize
        = (limit >> 3i32).wrapping_add(1i32 as (usize));
    'loop1: loop {
        if {
               limit2 = limit2.wrapping_sub(1 as (usize));
               limit2
           } != 0 {
            if BROTLI_UNALIGNED_LOAD64(
                   s2 as (*const ::std::os::raw::c_void)
               ) == BROTLI_UNALIGNED_LOAD64(
                        s1.offset(matched as (isize)) as (*const ::std::os::raw::c_void)
                    ) {
                s2 = s2.offset(8i32 as (isize));
                matched = matched.wrapping_add(8i32 as (usize));
                continue 'loop1;
            } else {
                break 'loop1;
            }
        } else {
            limit = (limit & 7i32 as (usize)).wrapping_add(1i32 as (usize));
            'loop3: loop {
                if {
                       limit = limit.wrapping_sub(1 as (usize));
                       limit
                   } != 0 {
                    if *s1.offset(matched as (isize)) as (i32) == *s2 as (i32) {
                        s2 = s2.offset(1 as (isize));
                        matched = matched.wrapping_add(1 as (usize));
                        continue 'loop3;
                    } else {
                        break 'loop3;
                    }
                } else {
                    return matched;
                }
            }
            return matched;
        }
    }
    let mut x
        : usize
        = BROTLI_UNALIGNED_LOAD64(
              s2 as (*const ::std::os::raw::c_void)
          ) ^ BROTLI_UNALIGNED_LOAD64(
                  s1.offset(matched as (isize)) as (*const ::std::os::raw::c_void)
              );
    let mut matching_bits : usize = unopt_ctzll(x) as (usize);
    matched = matched.wrapping_add(matching_bits >> 3i32);
    matched
}

unsafe extern fn EmitInsertLen(
    mut insertlen : u32, mut commands : *mut *mut u32
) {
    if insertlen < 6i32 as (u32) {
        **commands = insertlen;
    } else if insertlen < 130i32 as (u32) {
        let tail : u32 = insertlen.wrapping_sub(2i32 as (u32));
        let nbits
            : u32
            = Log2FloorNonZero(tail as (usize)).wrapping_sub(1u32);
        let prefix : u32 = tail >> nbits;
        let inscode
            : u32
            = (nbits << 1i32).wrapping_add(prefix).wrapping_add(2i32 as (u32));
        let extra : u32 = tail.wrapping_sub(prefix << nbits);
        **commands = inscode | extra << 8i32;
    } else if insertlen < 2114i32 as (u32) {
        let tail : u32 = insertlen.wrapping_sub(66i32 as (u32));
        let nbits : u32 = Log2FloorNonZero(tail as (usize));
        let code : u32 = nbits.wrapping_add(10i32 as (u32));
        let extra : u32 = tail.wrapping_sub(1u32 << nbits);
        **commands = code | extra << 8i32;
    } else if insertlen < 6210i32 as (u32) {
        let extra : u32 = insertlen.wrapping_sub(2114i32 as (u32));
        **commands = 21i32 as (u32) | extra << 8i32;
    } else if insertlen < 22594i32 as (u32) {
        let extra : u32 = insertlen.wrapping_sub(6210i32 as (u32));
        **commands = 22i32 as (u32) | extra << 8i32;
    } else {
        let extra : u32 = insertlen.wrapping_sub(22594i32 as (u32));
        **commands = 23i32 as (u32) | extra << 8i32;
    }
    *commands = (*commands).offset(1 as (isize));
}

unsafe extern fn EmitDistance(
    mut distance : u32, mut commands : *mut *mut u32
) {
    let mut d : u32 = distance.wrapping_add(3i32 as (u32));
    let mut nbits
        : u32
        = Log2FloorNonZero(d as (usize)).wrapping_sub(1i32 as (u32));
    let prefix : u32 = d >> nbits & 1i32 as (u32);
    let offset : u32 = (2i32 as (u32)).wrapping_add(prefix) << nbits;
    let distcode
        : u32
        = (2i32 as (u32)).wrapping_mul(
              nbits.wrapping_sub(1i32 as (u32))
          ).wrapping_add(
              prefix
          ).wrapping_add(
              80i32 as (u32)
          );
    let mut extra : u32 = d.wrapping_sub(offset);
    **commands = distcode | extra << 8i32;
    *commands = (*commands).offset(1 as (isize));
}

unsafe extern fn EmitCopyLenLastDistance(
    mut copylen : usize, mut commands : *mut *mut u32
) { if copylen < 12i32 as (usize) {
        **commands = copylen.wrapping_add(20i32 as (usize)) as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else if copylen < 72i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(8i32 as (usize));
        let nbits
            : usize
            = Log2FloorNonZero(tail).wrapping_sub(1i32 as (u32)) as (usize);
        let prefix : usize = tail >> nbits;
        let code
            : usize
            = (nbits << 1i32).wrapping_add(prefix).wrapping_add(
                  28i32 as (usize)
              );
        let extra : usize = tail.wrapping_sub(prefix << nbits);
        **commands = (code | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else if copylen < 136i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(8i32 as (usize));
        let code : usize = (tail >> 5i32).wrapping_add(54i32 as (usize));
        let extra : usize = tail & 31i32 as (usize);
        **commands = (code | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
        **commands = 64i32 as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else if copylen < 2120i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(72i32 as (usize));
        let nbits : usize = Log2FloorNonZero(tail) as (usize);
        let code : usize = nbits.wrapping_add(52i32 as (usize));
        let extra : usize = tail.wrapping_sub(1i32 as (usize) << nbits);
        **commands = (code | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
        **commands = 64i32 as (u32);
        *commands = (*commands).offset(1 as (isize));
    } else {
        let extra : usize = copylen.wrapping_sub(2120i32 as (usize));
        **commands = (63i32 as (usize) | extra << 8i32) as (u32);
        *commands = (*commands).offset(1 as (isize));
        **commands = 64i32 as (u32);
        *commands = (*commands).offset(1 as (isize));
    }
}

unsafe extern fn HashBytesAtOffset(
    mut v : usize, mut offset : i32, mut shift : usize
) -> u32 {
    if offset >= 0i32 {
        0i32;
    } else {
        __assert_fail(
            (*b"offset >= 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"HashBytesAtOffset\0").as_ptr()
        );
    }
    if offset <= 2i32 {
        0i32;
    } else {
        __assert_fail(
            (*b"offset <= 2\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"HashBytesAtOffset\0").as_ptr()
        );
    }
    let h
        : usize
        = (v >> 8i32 * offset << 16i32).wrapping_mul(
              kHashMul32 as (usize)
          );
    (h >> shift) as (u32)
}

unsafe extern fn EmitCopyLen(
    mut copylen : usize, mut commands : *mut *mut u32
) {
    if copylen < 10i32 as (usize) {
        **commands = copylen.wrapping_add(38i32 as (usize)) as (u32);
    } else if copylen < 134i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(6i32 as (usize));
        let nbits
            : usize
            = Log2FloorNonZero(tail).wrapping_sub(1i32 as (u32)) as (usize);
        let prefix : usize = tail >> nbits;
        let code
            : usize
            = (nbits << 1i32).wrapping_add(prefix).wrapping_add(
                  44i32 as (usize)
              );
        let extra : usize = tail.wrapping_sub(prefix << nbits);
        **commands = (code | extra << 8i32) as (u32);
    } else if copylen < 2118i32 as (usize) {
        let tail : usize = copylen.wrapping_sub(70i32 as (usize));
        let nbits : usize = Log2FloorNonZero(tail) as (usize);
        let code : usize = nbits.wrapping_add(52i32 as (usize));
        let extra : usize = tail.wrapping_sub(1i32 as (usize) << nbits);
        **commands = (code | extra << 8i32) as (u32);
    } else {
        let extra : usize = copylen.wrapping_sub(2118i32 as (usize));
        **commands = (63i32 as (usize) | extra << 8i32) as (u32);
    }
    *commands = (*commands).offset(1 as (isize));
}

unsafe extern fn CreateCommands(
    mut input : *const u8,
    mut block_size : usize,
    mut input_size : usize,
    mut base_ip : *const u8,
    mut table : *mut i32,
    mut table_bits : usize,
    mut literals : *mut *mut u8,
    mut commands : *mut *mut u32
) {
    let mut ip : *const u8 = input;
    let shift : usize = (64u32 as (usize)).wrapping_sub(table_bits);
    let mut ip_end : *const u8 = input.offset(block_size as (isize));
    let mut next_emit : *const u8 = input;
    let mut last_distance : i32 = -1i32;
    let kInputMarginBytes : usize = 16i32 as (usize);
    let kMinMatchLen : usize = 6i32 as (usize);
    if block_size >= kInputMarginBytes {
        let len_limit
            : usize
            = brotli_min_size_t(
                  block_size.wrapping_sub(kMinMatchLen),
                  input_size.wrapping_sub(kInputMarginBytes)
              );
        let mut ip_limit : *const u8 = input.offset(len_limit as (isize));
        let mut next_hash : u32;
        let mut goto_emit_remainder : i32 = 0i32;
        next_hash = Hash(
                        {
                            ip = ip.offset(1 as (isize));
                            ip
                        },
                        shift
                    );
        'loop2: loop {
            if goto_emit_remainder == 0 {
                let mut skip : u32 = 32i32 as (u32);
                let mut next_ip : *const u8 = ip;
                let mut candidate : *const u8;
                if next_emit < ip {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"next_emit < ip\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"CreateCommands\0").as_ptr()
                    );
                }
                'loop4: loop {
                    let mut hash : u32 = next_hash;
                    let mut bytes_between_hash_lookups
                        : u32
                        = ({
                               let _old = skip;
                               skip = skip.wrapping_add(1 as (u32));
                               _old
                           }) >> 5i32;
                    ip = next_ip;
                    if hash == Hash(ip,shift) {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"hash == Hash(ip, shift)\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"CreateCommands\0").as_ptr()
                        );
                    }
                    next_ip = ip.offset(bytes_between_hash_lookups as (isize));
                    if next_ip > ip_limit {
                        goto_emit_remainder = 1i32;
                    } else {
                        next_hash = Hash(next_ip,shift);
                        candidate = ip.offset(-(last_distance as (isize)));
                        if IsMatch(ip,candidate) != 0 {
                            if candidate < ip {
                                *table.offset(hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                                      base_ip as (isize)
                                                                  ) / ::std::mem::size_of::<u8>(
                                                                      ) as (isize)) as (i32);
                            }
                        }
                        candidate = base_ip.offset(
                                        *table.offset(hash as (isize)) as (isize)
                                    );
                        if candidate >= base_ip {
                            0i32;
                        } else {
                            __assert_fail(
                                (*b"candidate >= base_ip\0").as_ptr(),
                                file!().as_ptr(),
                                line!(),
                                (*b"CreateCommands\0").as_ptr()
                            );
                        }
                        if candidate < ip {
                            0i32;
                        } else {
                            __assert_fail(
                                (*b"candidate < ip\0").as_ptr(),
                                file!().as_ptr(),
                                line!(),
                                (*b"CreateCommands\0").as_ptr()
                            );
                        }
                        *table.offset(hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                              base_ip as (isize)
                                                          ) / ::std::mem::size_of::<u8>(
                                                              ) as (isize)) as (i32);
                        if IsMatch(ip,candidate) == 0 {
                            continue 'loop4;
                        }
                    }
                    if (ip as (isize)).wrapping_sub(
                           candidate as (isize)
                       ) / ::std::mem::size_of::<u8>(
                           ) as (isize) > (1i32 as (usize) << 18i32).wrapping_sub(
                                              16i32 as (usize)
                                          ) as (isize) && (goto_emit_remainder == 0) {
                        continue 'loop4;
                    } else {
                        break 'loop4;
                    }
                }
                if goto_emit_remainder != 0 {
                    break 'loop2;
                } else {
                    let mut base : *const u8 = ip;
                    let mut matched
                        : usize
                        = (6i32 as (usize)).wrapping_add(
                              FindMatchLengthWithLimit(
                                  candidate.offset(6i32 as (isize)),
                                  ip.offset(6i32 as (isize)),
                                  (((ip_end as (isize)).wrapping_sub(
                                        ip as (isize)
                                    ) / ::std::mem::size_of::<u8>(
                                        ) as (isize)) as (usize)).wrapping_sub(
                                      6i32 as (usize)
                                  )
                              )
                          );
                    let mut distance
                        : i32
                        = ((base as (isize)).wrapping_sub(
                               candidate as (isize)
                           ) / ::std::mem::size_of::<u8>() as (isize)) as (i32);
                    let mut insert
                        : i32
                        = ((base as (isize)).wrapping_sub(
                               next_emit as (isize)
                           ) / ::std::mem::size_of::<u8>() as (isize)) as (i32);
                    ip = ip.offset(matched as (isize));
                    if 0i32 == memcmp(
                                   base as (*const ::std::os::raw::c_void),
                                   candidate as (*const ::std::os::raw::c_void),
                                   matched
                               ) {
                        0i32;
                    } else {
                        __assert_fail(
                            (*b"0 == memcmp(base, candidate, matched)\0").as_ptr(),
                            file!().as_ptr(),
                            line!(),
                            (*b"CreateCommands\0").as_ptr()
                        );
                    }
                    EmitInsertLen(insert as (u32),commands);
                    memcpy(
                        *literals as (*mut ::std::os::raw::c_void),
                        next_emit as (*const ::std::os::raw::c_void),
                        insert as (usize)
                    );
                    *literals = (*literals).offset(insert as (isize));
                    if distance == last_distance {
                        **commands = 64i32 as (u32);
                        *commands = (*commands).offset(1 as (isize));
                    } else {
                        EmitDistance(distance as (u32),commands);
                        last_distance = distance;
                    }
                    EmitCopyLenLastDistance(matched,commands);
                    next_emit = ip;
                    if ip >= ip_limit {
                        goto_emit_remainder = 1i32;
                    } else {
                        let mut input_bytes
                            : usize
                            = BROTLI_UNALIGNED_LOAD64(
                                  ip.offset(-(5i32 as (isize))) as (*const ::std::os::raw::c_void)
                              );
                        let mut prev_hash
                            : u32
                            = HashBytesAtOffset(input_bytes,0i32,shift);
                        let mut cur_hash : u32;
                        *table.offset(
                             prev_hash as (isize)
                         ) = ((ip as (isize)).wrapping_sub(
                                  base_ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize) - 5i32 as (isize)) as (i32);
                        prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                        *table.offset(
                             prev_hash as (isize)
                         ) = ((ip as (isize)).wrapping_sub(
                                  base_ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize) - 4i32 as (isize)) as (i32);
                        prev_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                        *table.offset(
                             prev_hash as (isize)
                         ) = ((ip as (isize)).wrapping_sub(
                                  base_ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize) - 3i32 as (isize)) as (i32);
                        input_bytes = BROTLI_UNALIGNED_LOAD64(
                                          ip.offset(
                                              -(2i32 as (isize))
                                          ) as (*const ::std::os::raw::c_void)
                                      );
                        cur_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                        prev_hash = HashBytesAtOffset(input_bytes,0i32,shift);
                        *table.offset(
                             prev_hash as (isize)
                         ) = ((ip as (isize)).wrapping_sub(
                                  base_ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize) - 2i32 as (isize)) as (i32);
                        prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                        *table.offset(
                             prev_hash as (isize)
                         ) = ((ip as (isize)).wrapping_sub(
                                  base_ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize) - 1i32 as (isize)) as (i32);
                        candidate = base_ip.offset(
                                        *table.offset(cur_hash as (isize)) as (isize)
                                    );
                        *table.offset(cur_hash as (isize)) = ((ip as (isize)).wrapping_sub(
                                                                  base_ip as (isize)
                                                              ) / ::std::mem::size_of::<u8>(
                                                                  ) as (isize)) as (i32);
                        'loop18: loop {
                            if (ip as (isize)).wrapping_sub(
                                   candidate as (isize)
                               ) / ::std::mem::size_of::<u8>(
                                   ) as (isize) <= (1i32 as (usize) << 18i32).wrapping_sub(
                                                       16i32 as (usize)
                                                   ) as (isize) && (IsMatch(ip,candidate) != 0) {
                                let mut base : *const u8 = ip;
                                let mut matched
                                    : usize
                                    = (6i32 as (usize)).wrapping_add(
                                          FindMatchLengthWithLimit(
                                              candidate.offset(6i32 as (isize)),
                                              ip.offset(6i32 as (isize)),
                                              (((ip_end as (isize)).wrapping_sub(
                                                    ip as (isize)
                                                ) / ::std::mem::size_of::<u8>(
                                                    ) as (isize)) as (usize)).wrapping_sub(
                                                  6i32 as (usize)
                                              )
                                          )
                                      );
                                ip = ip.offset(matched as (isize));
                                last_distance = ((base as (isize)).wrapping_sub(
                                                     candidate as (isize)
                                                 ) / ::std::mem::size_of::<u8>(
                                                     ) as (isize)) as (i32);
                                if 0i32 == memcmp(
                                               base as (*const ::std::os::raw::c_void),
                                               candidate as (*const ::std::os::raw::c_void),
                                               matched
                                           ) {
                                    0i32;
                                } else {
                                    __assert_fail(
                                        (*b"0 == memcmp(base, candidate, matched)\0").as_ptr(),
                                        file!().as_ptr(),
                                        line!(),
                                        (*b"CreateCommands\0").as_ptr()
                                    );
                                }
                                EmitCopyLen(matched,commands);
                                EmitDistance(last_distance as (u32),commands);
                                next_emit = ip;
                                if ip >= ip_limit {
                                    goto_emit_remainder = 1i32;
                                } else {
                                    let mut input_bytes
                                        : usize
                                        = BROTLI_UNALIGNED_LOAD64(
                                              ip.offset(
                                                  -(5i32 as (isize))
                                              ) as (*const ::std::os::raw::c_void)
                                          );
                                    let mut prev_hash
                                        : u32
                                        = HashBytesAtOffset(input_bytes,0i32,shift);
                                    let mut cur_hash : u32;
                                    *table.offset(
                                         prev_hash as (isize)
                                     ) = ((ip as (isize)).wrapping_sub(
                                              base_ip as (isize)
                                          ) / ::std::mem::size_of::<u8>(
                                              ) as (isize) - 5i32 as (isize)) as (i32);
                                    prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                                    *table.offset(
                                         prev_hash as (isize)
                                     ) = ((ip as (isize)).wrapping_sub(
                                              base_ip as (isize)
                                          ) / ::std::mem::size_of::<u8>(
                                              ) as (isize) - 4i32 as (isize)) as (i32);
                                    prev_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                                    *table.offset(
                                         prev_hash as (isize)
                                     ) = ((ip as (isize)).wrapping_sub(
                                              base_ip as (isize)
                                          ) / ::std::mem::size_of::<u8>(
                                              ) as (isize) - 3i32 as (isize)) as (i32);
                                    input_bytes = BROTLI_UNALIGNED_LOAD64(
                                                      ip.offset(
                                                          -(2i32 as (isize))
                                                      ) as (*const ::std::os::raw::c_void)
                                                  );
                                    cur_hash = HashBytesAtOffset(input_bytes,2i32,shift);
                                    prev_hash = HashBytesAtOffset(input_bytes,0i32,shift);
                                    *table.offset(
                                         prev_hash as (isize)
                                     ) = ((ip as (isize)).wrapping_sub(
                                              base_ip as (isize)
                                          ) / ::std::mem::size_of::<u8>(
                                              ) as (isize) - 2i32 as (isize)) as (i32);
                                    prev_hash = HashBytesAtOffset(input_bytes,1i32,shift);
                                    *table.offset(
                                         prev_hash as (isize)
                                     ) = ((ip as (isize)).wrapping_sub(
                                              base_ip as (isize)
                                          ) / ::std::mem::size_of::<u8>(
                                              ) as (isize) - 1i32 as (isize)) as (i32);
                                    candidate = base_ip.offset(
                                                    *table.offset(cur_hash as (isize)) as (isize)
                                                );
                                    *table.offset(
                                         cur_hash as (isize)
                                     ) = ((ip as (isize)).wrapping_sub(
                                              base_ip as (isize)
                                          ) / ::std::mem::size_of::<u8>() as (isize)) as (i32);
                                    continue 'loop18;
                                }
                            } else {
                                break 'loop18;
                            }
                        }
                        if goto_emit_remainder == 0 {
                            next_hash = Hash(
                                            {
                                                ip = ip.offset(1 as (isize));
                                                ip
                                            },
                                            shift
                                        );
                            continue 'loop2;
                        } else {
                            continue 'loop2;
                        }
                    }
                }
            } else {
                break 'loop2;
            }
        }
    }
    if next_emit <= ip_end {
        0i32;
    } else {
        __assert_fail(
            (*b"next_emit <= ip_end\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"CreateCommands\0").as_ptr()
        );
    }
    if next_emit < ip_end {
        let insert
            : u32
            = ((ip_end as (isize)).wrapping_sub(
                   next_emit as (isize)
               ) / ::std::mem::size_of::<u8>() as (isize)) as (u32);
        EmitInsertLen(insert,commands);
        memcpy(
            *literals as (*mut ::std::os::raw::c_void),
            next_emit as (*const ::std::os::raw::c_void),
            insert as (usize)
        );
        *literals = (*literals).offset(insert as (isize));
    }
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < ::std::mem::size_of::<[f32; 256]>().wrapping_div(
               ::std::mem::size_of::<f32>()
           ) {
        kLog2Table[v] as (f64)
    } else {
        log2(v as (f64))
    }
}

unsafe extern fn ShannonEntropy(
    mut population : *const u32,
    mut size : usize,
    mut total : *mut usize
) -> f64 {
    let mut sum : usize = 0i32 as (usize);
    let mut retval : f64 = 0i32 as (f64);
    let mut population_end
        : *const u32
        = population.offset(size as (isize));
    let mut p : usize;
    let mut odd_number_of_elements_left : i32 = 0i32;
    if size & 1i32 as (usize) != 0 {
        odd_number_of_elements_left = 1i32;
    }
    'loop2: loop {
        if population < population_end {
            if odd_number_of_elements_left == 0 {
                p = *{
                         let _old = population;
                         population = population.offset(1 as (isize));
                         _old
                     } as (usize);
                sum = sum.wrapping_add(p);
                retval = retval - p as (f64) * FastLog2(p);
            }
            odd_number_of_elements_left = 0i32;
            p = *{
                     let _old = population;
                     population = population.offset(1 as (isize));
                     _old
                 } as (usize);
            sum = sum.wrapping_add(p);
            retval = retval - p as (f64) * FastLog2(p);
            continue 'loop2;
        } else {
            break 'loop2;
        }
    }
    if sum != 0 {
        retval = retval + sum as (f64) * FastLog2(sum);
    }
    *total = sum;
    retval
}

unsafe extern fn BitsEntropy(
    mut population : *const u32, mut size : usize
) -> f64 {
    let mut sum : usize;
    let mut retval
        : f64
        = ShannonEntropy(population,size,&mut sum as (*mut usize));
    if retval < sum as (f64) {
        retval = sum as (f64);
    }
    retval
}

unsafe extern fn ShouldCompress(
    mut input : *const u8,
    mut input_size : usize,
    mut num_literals : usize
) -> i32 {
    let mut corpus_size : f64 = input_size as (f64);
    if num_literals as (f64) < 0.98f64 * corpus_size {
        1i32
    } else {
        let mut literal_histo
            : [u32; 256]
            = [   0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32),
                  0i32 as (u32)
              ];
        let max_total_bit_cost
            : f64
            = corpus_size * 8i32 as (f64) * 0.98f64 / 43i32 as (f64);
        let mut i : usize;
        i = 0i32 as (usize);
        'loop2: loop {
            if i < input_size {
                {
                    let _rhs = 1;
                    let _lhs
                        = &mut literal_histo[*input.offset(i as (isize)) as (usize)];
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                i = i.wrapping_add(43i32 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        if !!(BitsEntropy(
                  literal_histo.as_mut_ptr() as (*const u32),
                  256i32 as (usize)
              ) < max_total_bit_cost) {
            1i32
        } else {
            0i32
        }
    }
}

unsafe extern fn BROTLI_UNALIGNED_STORE64(
    mut p : *mut ::std::os::raw::c_void, mut v : usize
) {
    memcpy(
        p,
        &mut v as (*mut usize) as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<usize>()
    );
}

unsafe extern fn BrotliWriteBits(
    mut n_bits : usize,
    mut bits : usize,
    mut pos : *mut usize,
    mut array : *mut u8
) {
    let mut p
        : *mut u8
        = &mut *array.offset((*pos >> 3i32) as (isize)) as (*mut u8);
    let mut v : usize = *p as (usize);
    if bits >> n_bits == 0i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"(bits >> n_bits) == 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliWriteBits\0").as_ptr()
        );
    }
    if n_bits <= 56i32 as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"n_bits <= 56\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliWriteBits\0").as_ptr()
        );
    }
    v = v | bits << (*pos & 7i32 as (usize));
    BROTLI_UNALIGNED_STORE64(p as (*mut ::std::os::raw::c_void),v);
    *pos = (*pos).wrapping_add(n_bits);
}

unsafe extern fn BrotliStoreMetaBlockHeader(
    mut len : usize,
    mut is_uncompressed : i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut nibbles : usize = 6i32 as (usize);
    BrotliWriteBits(
        1i32 as (usize),
        0i32 as (usize),
        storage_ix,
        storage
    );
    if len <= (1u32 << 16i32) as (usize) {
        nibbles = 4i32 as (usize);
    } else if len <= (1u32 << 20i32) as (usize) {
        nibbles = 5i32 as (usize);
    }
    BrotliWriteBits(
        2i32 as (usize),
        nibbles.wrapping_sub(4i32 as (usize)),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        nibbles.wrapping_mul(4i32 as (usize)),
        len.wrapping_sub(1i32 as (usize)),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        1i32 as (usize),
        is_uncompressed as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn EmitUncompressedMetaBlock(
    mut input : *const u8,
    mut input_size : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliStoreMetaBlockHeader(input_size,1i32,storage_ix,storage);
    *storage_ix = (*storage_ix).wrapping_add(
                      7u32 as (usize)
                  ) & !7u32 as (usize);
    memcpy(
        &mut *storage.offset(
                  (*storage_ix >> 3i32) as (isize)
              ) as (*mut u8) as (*mut ::std::os::raw::c_void),
        input as (*const ::std::os::raw::c_void),
        input_size
    );
    *storage_ix = (*storage_ix).wrapping_add(input_size << 3i32);
    *storage.offset((*storage_ix >> 3i32) as (isize)) = 0i32 as (u8);
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}

unsafe extern fn BuildAndStoreCommandPrefixCode(
    mut histogram : *const u32,
    mut depth : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut tree : [HuffmanTree; 129];
    let mut cmd_depth
        : [u8; 704]
        = [   0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8)
          ];
    let mut cmd_bits : [u16; 64];
    BrotliCreateHuffmanTree(
        histogram,
        64i32 as (usize),
        15i32,
        tree.as_mut_ptr(),
        depth
    );
    BrotliCreateHuffmanTree(
        &*histogram.offset(64i32 as (isize)) as (*const u32),
        64i32 as (usize),
        14i32,
        tree.as_mut_ptr(),
        &mut *depth.offset(64i32 as (isize)) as (*mut u8)
    );
    memcpy(
        cmd_depth.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        depth.offset(24i32 as (isize)) as (*const ::std::os::raw::c_void),
        24i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            24i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            32i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(48i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            40i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(8i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            48i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(56i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            56i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(16i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    BrotliConvertBitDepthsToSymbols(
        cmd_depth.as_mut_ptr() as (*const u8),
        64i32 as (usize),
        cmd_bits.as_mut_ptr()
    );
    memcpy(
        bits as (*mut ::std::os::raw::c_void),
        cmd_bits.as_mut_ptr().offset(
            24i32 as (isize)
        ) as (*const ::std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(8i32 as (isize)) as (*mut ::std::os::raw::c_void),
        cmd_bits.as_mut_ptr().offset(
            40i32 as (isize)
        ) as (*const ::std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(16i32 as (isize)) as (*mut ::std::os::raw::c_void),
        cmd_bits.as_mut_ptr().offset(
            56i32 as (isize)
        ) as (*const ::std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(24i32 as (isize)) as (*mut ::std::os::raw::c_void),
        cmd_bits.as_mut_ptr() as (*const ::std::os::raw::c_void),
        48i32 as (usize)
    );
    memcpy(
        bits.offset(48i32 as (isize)) as (*mut ::std::os::raw::c_void),
        cmd_bits.as_mut_ptr().offset(
            32i32 as (isize)
        ) as (*const ::std::os::raw::c_void),
        16i32 as (usize)
    );
    memcpy(
        bits.offset(56i32 as (isize)) as (*mut ::std::os::raw::c_void),
        cmd_bits.as_mut_ptr().offset(
            48i32 as (isize)
        ) as (*const ::std::os::raw::c_void),
        16i32 as (usize)
    );
    BrotliConvertBitDepthsToSymbols(
        &mut *depth.offset(64i32 as (isize)) as (*mut u8) as (*const u8),
        64i32 as (usize),
        &mut *bits.offset(64i32 as (isize)) as (*mut u16)
    );
    let mut i : usize;
    memset(
        cmd_depth.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        64i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        depth.offset(24i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            64i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(32i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            128i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(40i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            192i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(48i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    memcpy(
        cmd_depth.as_mut_ptr().offset(
            384i32 as (isize)
        ) as (*mut ::std::os::raw::c_void),
        depth.offset(56i32 as (isize)) as (*const ::std::os::raw::c_void),
        8i32 as (usize)
    );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 8i32 as (usize) {
            cmd_depth[
                (128i32 as (usize)).wrapping_add((8i32 as (usize)).wrapping_mul(i))
            ] = *depth.offset(i as (isize));
            cmd_depth[
                (256i32 as (usize)).wrapping_add((8i32 as (usize)).wrapping_mul(i))
            ] = *depth.offset((8i32 as (usize)).wrapping_add(i) as (isize));
            cmd_depth[
                (448i32 as (usize)).wrapping_add((8i32 as (usize)).wrapping_mul(i))
            ] = *depth.offset((16i32 as (usize)).wrapping_add(i) as (isize));
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    BrotliStoreHuffmanTree(
        cmd_depth.as_mut_ptr() as (*const u8),
        704i32 as (usize),
        tree.as_mut_ptr(),
        storage_ix,
        storage
    );
    BrotliStoreHuffmanTree(
        &mut *depth.offset(64i32 as (isize)) as (*mut u8) as (*const u8),
        64i32 as (usize),
        tree.as_mut_ptr(),
        storage_ix,
        storage
    );
}

unsafe extern fn StoreCommands(
    mut m : *mut MemoryManager,
    mut literals : *const u8,
    num_literals : usize,
    mut commands : *const u32,
    num_commands : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    static mut kNumExtraBits
        : [u32; 128]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              1i32 as (u32),
              1i32 as (u32),
              2i32 as (u32),
              2i32 as (u32),
              3i32 as (u32),
              3i32 as (u32),
              4i32 as (u32),
              4i32 as (u32),
              5i32 as (u32),
              5i32 as (u32),
              6i32 as (u32),
              7i32 as (u32),
              8i32 as (u32),
              9i32 as (u32),
              10i32 as (u32),
              12i32 as (u32),
              14i32 as (u32),
              24i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              1i32 as (u32),
              1i32 as (u32),
              2i32 as (u32),
              2i32 as (u32),
              3i32 as (u32),
              3i32 as (u32),
              4i32 as (u32),
              4i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              1i32 as (u32),
              1i32 as (u32),
              2i32 as (u32),
              2i32 as (u32),
              3i32 as (u32),
              3i32 as (u32),
              4i32 as (u32),
              4i32 as (u32),
              5i32 as (u32),
              5i32 as (u32),
              6i32 as (u32),
              7i32 as (u32),
              8i32 as (u32),
              9i32 as (u32),
              10i32 as (u32),
              24i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              1i32 as (u32),
              1i32 as (u32),
              2i32 as (u32),
              2i32 as (u32),
              3i32 as (u32),
              3i32 as (u32),
              4i32 as (u32),
              4i32 as (u32),
              5i32 as (u32),
              5i32 as (u32),
              6i32 as (u32),
              6i32 as (u32),
              7i32 as (u32),
              7i32 as (u32),
              8i32 as (u32),
              8i32 as (u32),
              9i32 as (u32),
              9i32 as (u32),
              10i32 as (u32),
              10i32 as (u32),
              11i32 as (u32),
              11i32 as (u32),
              12i32 as (u32),
              12i32 as (u32),
              13i32 as (u32),
              13i32 as (u32),
              14i32 as (u32),
              14i32 as (u32),
              15i32 as (u32),
              15i32 as (u32),
              16i32 as (u32),
              16i32 as (u32),
              17i32 as (u32),
              17i32 as (u32),
              18i32 as (u32),
              18i32 as (u32),
              19i32 as (u32),
              19i32 as (u32),
              20i32 as (u32),
              20i32 as (u32),
              21i32 as (u32),
              21i32 as (u32),
              22i32 as (u32),
              22i32 as (u32),
              23i32 as (u32),
              23i32 as (u32),
              24i32 as (u32),
              24i32 as (u32)
          ];
    static mut kInsertOffset
        : [u32; 24]
        = [   0i32 as (u32),
              1i32 as (u32),
              2i32 as (u32),
              3i32 as (u32),
              4i32 as (u32),
              5i32 as (u32),
              6i32 as (u32),
              8i32 as (u32),
              10i32 as (u32),
              14i32 as (u32),
              18i32 as (u32),
              26i32 as (u32),
              34i32 as (u32),
              50i32 as (u32),
              66i32 as (u32),
              98i32 as (u32),
              130i32 as (u32),
              194i32 as (u32),
              322i32 as (u32),
              578i32 as (u32),
              1090i32 as (u32),
              2114i32 as (u32),
              6210i32 as (u32),
              22594i32 as (u32)
          ];
    let mut lit_depths : [u8; 256];
    let mut lit_bits : [u16; 256];
    let mut lit_histo
        : [u32; 256]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut cmd_depths
        : [u8; 128]
        = [   0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8)
          ];
    let mut cmd_bits
        : [u16; 128]
        = [   0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16),
              0i32 as (u16)
          ];
    let mut cmd_histo
        : [u32; 128]
        = [   0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32),
              0i32 as (u32)
          ];
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < num_literals {
            {
                let _rhs = 1;
                let _lhs
                    = &mut lit_histo[*literals.offset(i as (isize)) as (usize)];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    BrotliBuildAndStoreHuffmanTreeFast(
        m,
        lit_histo.as_mut_ptr() as (*const u32),
        num_literals,
        8i32 as (usize),
        lit_depths.as_mut_ptr(),
        lit_bits.as_mut_ptr(),
        storage_ix,
        storage
    );
    if !(0i32 == 0) {
    } else {
        i = 0i32 as (usize);
        'loop4: loop {
            if i < num_commands {
                let code : u32 = *commands.offset(i as (isize)) & 0xffi32 as (u32);
                if code < 128i32 as (u32) {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"code < 128\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"StoreCommands\0").as_ptr()
                    );
                }
                {
                    let _rhs = 1;
                    let _lhs = &mut cmd_histo[code as (usize)];
                    *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        {
            let _rhs = 1i32;
            let _lhs = &mut cmd_histo[1i32 as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        {
            let _rhs = 1i32;
            let _lhs = &mut cmd_histo[2i32 as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        {
            let _rhs = 1i32;
            let _lhs = &mut cmd_histo[64i32 as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        {
            let _rhs = 1i32;
            let _lhs = &mut cmd_histo[84i32 as (usize)];
            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
        }
        BuildAndStoreCommandPrefixCode(
            cmd_histo.as_mut_ptr() as (*const u32),
            cmd_depths.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        i = 0i32 as (usize);
        'loop6: loop {
            if i < num_commands {
                let cmd : u32 = *commands.offset(i as (isize));
                let code : u32 = cmd & 0xffi32 as (u32);
                let extra : u32 = cmd >> 8i32;
                if code < 128i32 as (u32) {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"code < 128\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"StoreCommands\0").as_ptr()
                    );
                }
                BrotliWriteBits(
                    cmd_depths[code as (usize)] as (usize),
                    cmd_bits[code as (usize)] as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    kNumExtraBits[code as (usize)] as (usize),
                    extra as (usize),
                    storage_ix,
                    storage
                );
                if code < 24i32 as (u32) {
                    let insert
                        : u32
                        = kInsertOffset[code as (usize)].wrapping_add(extra);
                    let mut j : u32;
                    j = 0i32 as (u32);
                    'loop10: loop {
                        if j < insert {
                            let lit : u8 = *literals;
                            BrotliWriteBits(
                                lit_depths[lit as (usize)] as (usize),
                                lit_bits[lit as (usize)] as (usize),
                                storage_ix,
                                storage
                            );
                            literals = literals.offset(1 as (isize));
                            j = j.wrapping_add(1 as (u32));
                            continue 'loop10;
                        } else {
                            break 'loop10;
                        }
                    }
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
    }
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut table_bits : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut base_ip : *const u8 = input;
    is_last;
    'loop1: loop {
        if input_size > 0i32 as (usize) {
            let mut block_size
                : usize
                = brotli_min_size_t(input_size,kCompressFragmentTwoPassBlockSize);
            let mut commands : *mut u32 = command_buf;
            let mut literals : *mut u8 = literal_buf;
            let mut num_literals : usize;
            CreateCommands(
                input,
                block_size,
                input_size,
                base_ip,
                table,
                table_bits,
                &mut literals as (*mut *mut u8),
                &mut commands as (*mut *mut u32)
            );
            num_literals = ((literals as (isize)).wrapping_sub(
                                literal_buf as (isize)
                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
            if ShouldCompress(input,block_size,num_literals) != 0 {
                let num_commands
                    : usize
                    = ((commands as (isize)).wrapping_sub(
                           command_buf as (isize)
                       ) / ::std::mem::size_of::<u32>() as (isize)) as (usize);
                BrotliStoreMetaBlockHeader(block_size,0i32,storage_ix,storage);
                BrotliWriteBits(
                    13i32 as (usize),
                    0i32 as (usize),
                    storage_ix,
                    storage
                );
                StoreCommands(
                    m,
                    literal_buf as (*const u8),
                    num_literals,
                    command_buf as (*const u32),
                    num_commands,
                    storage_ix,
                    storage
                );
                if !(0i32 == 0) {
                    break 'loop1;
                }
            } else {
                EmitUncompressedMetaBlock(input,block_size,storage_ix,storage);
            }
            input = input.offset(block_size as (isize));
            input_size = input_size.wrapping_sub(block_size);
            continue 'loop1;
        }
    }
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl8(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        8i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl9(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        9i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl10(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        10i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl11(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        11i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl12(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        12i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl13(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        13i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl14(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        14i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl15(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        15i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl16(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        16i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliCompressFragmentTwoPassImpl17(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliCompressFragmentTwoPassImpl(
        m,
        input,
        input_size,
        is_last,
        command_buf,
        literal_buf,
        table,
        17i32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn RewindBitPosition(
    new_storage_ix : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let bitpos : usize = new_storage_ix & 7i32 as (usize);
    let mask
        : usize
        = (1u32 << bitpos).wrapping_sub(1i32 as (u32)) as (usize);
    {
        let _rhs = mask as (u8);
        let _lhs
            = &mut *storage.offset((new_storage_ix >> 3i32) as (isize));
        *_lhs = (*_lhs as (i32) & _rhs as (i32)) as (u8);
    }
    *storage_ix = new_storage_ix;
}

#[no_mangle]
pub unsafe extern fn BrotliCompressFragmentTwoPass(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut input_size : usize,
    mut is_last : i32,
    mut command_buf : *mut u32,
    mut literal_buf : *mut u8,
    mut table : *mut i32,
    mut table_size : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let initial_storage_ix : usize = *storage_ix;
    let table_bits : usize = Log2FloorNonZero(table_size) as (usize);
    if table_bits == 17i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl17(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 16i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl16(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 15i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl15(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 14i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl14(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 13i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl13(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 12i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl12(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 11i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl11(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 10i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl10(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 9i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl9(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if table_bits == 8i32 as (usize) {
        BrotliCompressFragmentTwoPassImpl8(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            storage_ix,
            storage
        );
    } else if 0i32 != 0 {
        0i32;
    } else {
        __assert_fail(
            (*b"0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliCompressFragmentTwoPass\0").as_ptr()
        );
    }
    if (*storage_ix).wrapping_sub(
           initial_storage_ix
       ) > (31i32 as (usize)).wrapping_add(input_size << 3i32) {
        RewindBitPosition(initial_storage_ix,storage_ix,storage);
        EmitUncompressedMetaBlock(input,input_size,storage_ix,storage);
    }
    if is_last != 0 {
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            1i32 as (usize),
            1i32 as (usize),
            storage_ix,
            storage
        );
        *storage_ix = (*storage_ix).wrapping_add(
                          7u32 as (usize)
                      ) & !7u32 as (usize);
    }
}