use super::Array;
use crate::{define_composed_type, define_constants, define_model_type};

mod raw_elf64 {
    pub type Half = u16;
    pub type Word = u32;
    pub type XWord = u64;
    pub type Addr = usize; // FIXME: It is ok?
    pub type Off = usize; // FIXME: same
}

use raw_elf64::*; // FIXME: support 32bit

pub mod elf_header {
    use super::*;
    define_model_type!(
        #[derive(PartialEq, Eq)]
        pub struct Magic(Array<u8, 16>), []
    );

    impl core::fmt::Display for Magic {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "[")?;
            for x in &self.0 .0 {
                if x.is_ascii_alphanumeric() {
                    write!(f, "{:>2}, ", *x as char)?;
                } else {
                    write!(f, "0x{:0>2x}, ", x)?;
                }
            }
            write!(f, "]")?;

            Ok(())
        }
    }

    define_model_type!(
        #[derive(PartialEq, Eq)]
        pub struct Type(Half),
        pub
        [
    #define ET_NONE		0
    #define ET_REL		1
    #define ET_EXEC		2
    #define ET_DYN		3
    #define ET_CORE		4
    #define	ET_NUM		5
    #define ET_LOOS		0xfe00
    #define ET_HIOS		0xfeff
    #define ET_LOPROC	0xff00
    #define ET_HIPROC	0xffff
        ]
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq)]
        pub struct Machine(Half),
        pub
        [
    #define EM_NONE		 0
    #define EM_M32		 1
    #define EM_SPARC	 2
    #define EM_386		 3
    #define EM_68K		 4
    #define EM_88K		 5
    #define EM_IAMCU	 6
    #define EM_860		 7
    #define EM_MIPS		 8
    #define EM_S370		 9
    #define EM_MIPS_RS3_LE	10
    #define EM_PARISC	15
    #define EM_VPP500	17
    #define EM_SPARC32PLUS	18
    #define EM_960		19
    #define EM_PPC		20
    #define EM_PPC64	21
    #define EM_S390		22
    #define EM_SPU		23
    #define EM_V800		36
    #define EM_FR20		37
    #define EM_RH32		38
    #define EM_RCE		39
    #define EM_ARM		40
    #define EM_FAKE_ALPHA	41
    #define EM_SH		42
    #define EM_SPARCV9	43
    #define EM_TRICORE	44
    #define EM_ARC		45
    #define EM_H8_300	46
    #define EM_H8_300H	47
    #define EM_H8S		48
    #define EM_H8_500	49
    #define EM_IA_64	50
    #define EM_MIPS_X	51
    #define EM_COLDFIRE	52
    #define EM_68HC12	53
    #define EM_MMA		54
    #define EM_PCP		55
    #define EM_NCPU		56
    #define EM_NDR1		57
    #define EM_STARCORE	58
    #define EM_ME16		59
    #define EM_ST100	60
    #define EM_TINYJ	61
    #define EM_X86_64	62
    #define EM_PDSP		63
    #define EM_PDP10	64
    #define EM_PDP11	65
    #define EM_FX66		66
    #define EM_ST9PLUS	67
    #define EM_ST7		68
    #define EM_68HC16	69
    #define EM_68HC11	70
    #define EM_68HC08	71
    #define EM_68HC05	72
    #define EM_SVX		73
    #define EM_ST19		74
    #define EM_VAX		75
    #define EM_CRIS		76
    #define EM_JAVELIN	77
    #define EM_FIREPATH	78
    #define EM_ZSP		79
    #define EM_MMIX		80
    #define EM_HUANY	81
    #define EM_PRISM	82
    #define EM_AVR		83
    #define EM_FR30		84
    #define EM_D10V		85
    #define EM_D30V		86
    #define EM_V850		87
    #define EM_M32R		88
    #define EM_MN10300	89
    #define EM_MN10200	90
    #define EM_PJ		91
    #define EM_OPENRISC	92
    #define EM_ARC_COMPACT	93
    #define EM_XTENSA	94
    #define EM_VIDEOCORE	95
    #define EM_TMM_GPP	96
    #define EM_NS32K	97
    #define EM_TPC		98
    #define EM_SNP1K	99
    #define EM_ST200	100
    #define EM_IP2K		101
    #define EM_MAX		102
    #define EM_CR		103
    #define EM_F2MC16	104
    #define EM_MSP430	105
    #define EM_BLACKFIN	106
    #define EM_SE_C33	107
    #define EM_SEP		108
    #define EM_ARCA		109
    #define EM_UNICORE	110
    #define EM_EXCESS	111
    #define EM_DXP		112
    #define EM_ALTERA_NIOS2 113
    #define EM_CRX		114
    #define EM_XGATE	115
    #define EM_C166		116
    #define EM_M16C		117
    #define EM_DSPIC30F	118
    #define EM_CE		119
    #define EM_M32C		120
    #define EM_TSK3000	131
    #define EM_RS08		132
    #define EM_SHARC	133
    #define EM_ECOG2	134
    #define EM_SCORE7	135
    #define EM_DSP24	136
    #define EM_VIDEOCORE3	137
    #define EM_LATTICEMICO32 138
    #define EM_SE_C17	139
    #define EM_TI_C6000	140
    #define EM_TI_C2000	141
    #define EM_TI_C5500	142
    #define EM_TI_ARP32	143
    #define EM_TI_PRU	144
    #define EM_MMDSP_PLUS	160
    #define EM_CYPRESS_M8C	161
    #define EM_R32C		162
    #define EM_TRIMEDIA	163
    #define EM_QDSP6	164
    #define EM_8051		165
    #define EM_STXP7X	166
    #define EM_NDS32	167
    #define EM_ECOG1X	168
    #define EM_MAXQ30	169
    #define EM_XIMO16	170
    #define EM_MANIK	171
    #define EM_CRAYNV2	172
    #define EM_RX		173
    #define EM_METAG	174
    #define EM_MCST_ELBRUS	175
    #define EM_ECOG16	176
    #define EM_CR16		177
    #define EM_ETPU		178
    #define EM_SLE9X	179
    #define EM_L10M		180
    #define EM_K10M		181
    #define EM_AARCH64	183
    #define EM_AVR32	185
    #define EM_STM8		186
    #define EM_TILE64	187
    #define EM_TILEPRO	188
    #define EM_MICROBLAZE	189
    #define EM_CUDA		190
    #define EM_TILEGX	191
    #define EM_CLOUDSHIELD	192
    #define EM_COREA_1ST	193
    #define EM_COREA_2ND	194
    #define EM_ARCV2	195
    #define EM_OPEN8	196
    #define EM_RL78		197
    #define EM_VIDEOCORE5	198
    #define EM_78KOR	199
    #define EM_56800EX	200
    #define EM_BA1		201
    #define EM_BA2		202
    #define EM_XCORE	203
    #define EM_MCHP_PIC	204
    #define EM_INTELGT	205
    #define EM_KM32		210
    #define EM_KMX32	211
    #define EM_EMX16	212
    #define EM_EMX8		213
    #define EM_KVARC	214
    #define EM_CDP		215
    #define EM_COGE		216
    #define EM_COOL		217
    #define EM_NORC		218
    #define EM_CSR_KALIMBA	219
    #define EM_Z80		220
    #define EM_VISIUM	221
    #define EM_FT32		222
    #define EM_MOXIE	223
    #define EM_AMDGPU	224
    #define EM_RISCV	243
    #define EM_BPF		247
    #define EM_CSKY		252
    #define EM_NUM		253
    #define EM_ALPHA	0x9026
        ]
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq)]
        pub struct Version(Word),
        pub
        [
    #define EV_NONE		0
    #define EV_CURRENT	1
    #define EV_NUM		2
        ]
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
        pub struct Entry(Addr),
        []
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
        pub struct PhOff(Off),
        []
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
        pub struct ShOff(Off),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Flag(Word),
        pub
        [
    #define EF_SPARCV9_MM		3
    #define EF_SPARCV9_TSO		0
    #define EF_SPARCV9_PSO		1
    #define EF_SPARCV9_RMO		2
    #define EF_SPARC_LEDATA		0x800000
    #define EF_SPARC_EXT_MASK	0xFFFF00
    #define EF_SPARC_32PLUS		0x000100
    #define EF_SPARC_SUN_US1	0x000200
    #define EF_SPARC_HAL_R1		0x000400
    #define EF_SPARC_SUN_US3	0x000800
        ]
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct EhSize(Half),
        []
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct PhEntrySize(Half),
        []
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct PhNum(Half),
        [
    #define PN_XNUM		0xffff
        ]
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct ShEntrySize(Half),
        []
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct ShNum(Half),
        []
        display_implementation = true
    );
    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct ShStrIndex(Half),
        [
    #define SHN_UNDEF	0		/* Undefined section */
    #define SHN_LORESERVE	0xff00		/* Start of reserved indices */
    #define SHN_LOPROC	0xff00		/* Start of processor-specific */
    #define SHN_BEFORE	0xff00		/* Order section before all others
                           (Solaris).  */
    #define SHN_AFTER	0xff01		/* Order section after all others
                           (Solaris).  */
    #define SHN_HIPROC	0xff1f		/* End of processor-specific */
    #define SHN_LOOS	0xff20		/* Start of OS-specific */
    #define SHN_HIOS	0xff3f		/* End of OS-specific */
    #define SHN_ABS		0xfff1		/* Associated symbol is absolute */
    #define SHN_COMMON	0xfff2		/* Associated symbol is common */
    #define SHN_XINDEX	0xffff		/* Index is in extra table.  */
    #define SHN_HIRESERVE	0xffff		/* End of reserved indices */
        ]
        display_implementation = true
    );

    define_composed_type!(
        pub struct Header {
            /// Magic number and other info
            pub e_ident: Option<Magic>,

            /// Object file type
            pub e_type: Option<Type>,

            /// Architecture
            pub e_machine: Option<Machine>,

            /// Object file version
            pub e_version: Option<Version>,

            /// Entry point virtual address
            pub e_entry: Option<Entry>,

            /// Program header table file offset
            pub e_phoff: Option<PhOff>,

            /// Section header table file offset
            pub e_shoff: Option<ShOff>,

            /// Processor-specific flags
            pub e_flags: Option<Flag>,

            /// ELF header size in bytes
            pub e_ehsize: Option<EhSize>,

            /// Program header table entry size
            pub e_phentsize: Option<PhEntrySize>,

            /// Program header table entry count
            pub e_phnum: Option<PhNum>,

            /// Section header table entry size
            pub e_shentsize: Option<ShEntrySize>,

            /// Section header table entry count
            pub e_shnum: Option<ShNum>,

            /// Section header string table index
            pub e_shstrndx: Option<ShStrIndex>,
        },
        display_implementation = true
    );
}

pub mod section_header {
    use super::*;

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Name(Word),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Type(Word),
        pub
        [
    #define SHT_NULL	  0		/* Section header table entry unused */
    #define SHT_PROGBITS	  1		/* Program data */
    #define SHT_SYMTAB	  2		/* Symbol table */
    #define SHT_STRTAB	  3		/* String table */
    #define SHT_RELA	  4		/* Relocation entries with addends */
    #define SHT_HASH	  5		/* Symbol hash table */
    #define SHT_DYNAMIC	  6		/* Dynamic linking information */
    #define SHT_NOTE	  7		/* Notes */
    #define SHT_NOBITS	  8		/* Program space with no data (bss) */
    #define SHT_REL		  9		/* Relocation entries, no addends */
    #define SHT_SHLIB	  10		/* Reserved */
    #define SHT_DYNSYM	  11		/* Dynamic linker symbol table */
    #define SHT_INIT_ARRAY	  14		/* Array of constructors */
    #define SHT_FINI_ARRAY	  15		/* Array of destructors */
    #define SHT_PREINIT_ARRAY 16		/* Array of pre-constructors */
    #define SHT_GROUP	  17		/* Section group */
    #define SHT_SYMTAB_SHNDX  18		/* Extended section indices */
    #define	SHT_NUM		  19		/* Number of defined types.  */
    #define SHT_LOOS	  0x60000000	/* Start OS-specific.  */
    #define SHT_GNU_ATTRIBUTES 0x6ffffff5	/* Object attributes.  */
    #define SHT_GNU_HASH	  0x6ffffff6	/* GNU-style hash table.  */
    #define SHT_GNU_LIBLIST	  0x6ffffff7	/* Prelink library list */
    #define SHT_CHECKSUM	  0x6ffffff8	/* Checksum for DSO content.  */
    #define SHT_LOSUNW	  0x6ffffffa	/* Sun-specific low bound.  */
    #define SHT_SUNW_move	  0x6ffffffa
    #define SHT_SUNW_COMDAT   0x6ffffffb
    #define SHT_SUNW_syminfo  0x6ffffffc
    #define SHT_GNU_verdef	  0x6ffffffd	/* Version definition section.  */
    #define SHT_GNU_verneed	  0x6ffffffe	/* Version needs section.  */
    #define SHT_GNU_versym	  0x6fffffff	/* Version symbol table.  */
    #define SHT_HISUNW	  0x6fffffff	/* Sun-specific high bound.  */
    #define SHT_HIOS	  0x6fffffff	/* End OS-specific type */
    #define SHT_LOPROC	  0x70000000	/* Start of processor-specific */
    #define SHT_HIPROC	  0x7fffffff	/* End of processor-specific */
    #define SHT_LOUSER	  0x80000000	/* Start of application-specific */
    #define SHT_HIUSER	  0x8fffffff	/* End of application-specific */
        ]
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Flags(XWord),
        pub
        [
    #define SHF_WRITE	     (1 << 0)	/* Writable */ #define SHF_ALLOC	     (1 << 1)	/* Occupies memory during execution */
    #define SHF_EXECINSTR	     (1 << 2)	/* Executable */
    #define SHF_MERGE	     (1 << 4)	/* Might be merged */
    #define SHF_STRINGS	     (1 << 5)	/* Contains nul-terminated strings */
    #define SHF_INFO_LINK	     (1 << 6)	/* `sh_info' contains SHT index */
    #define SHF_LINK_ORDER	     (1 << 7)	/* Preserve order after combining */
    #define SHF_OS_NONCONFORMING (1 << 8)	/* Non-standard OS specific handling
                           required */
    #define SHF_GROUP	     (1 << 9)	/* Section is member of a group.  */
    #define SHF_TLS		     (1 << 10)	/* Section hold thread-local data.  */
    #define SHF_COMPRESSED	     (1 << 11)	/* Section with compressed data. */
    #define SHF_MASKOS	     0x0ff00000	/* OS-specific.  */
    #define SHF_MASKPROC	     0xf0000000	/* Processor-specific */
    #define SHF_GNU_RETAIN	     (1 << 21)  /* Not to be GCed by linker.  */
    #define SHF_ORDERED	     (1 << 30)	/* Special ordering requirement
                           (Solaris).  */
    #define SHF_EXCLUDE	     (1 << 31)	/* Section is excluded unless
                           referenced or allocated (Solaris).*/
        ]
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct VirtualAddress(Addr),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct FileOffset(Off),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Size(XWord),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Link(Word),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Info(Word),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Align(XWord),
        []
        display_implementation = true
    );

    define_model_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct EntrySize(XWord),
        []
        display_implementation = true
    );

    define_composed_type!(
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub struct Header {
            /// Section name (string tbl index)
            sh_name: Option<Name>,

            /// Section type  
            sh_type: Option<Type>,

            /// Section flags
            sh_flags: Option<Flags>,

            /// Section virtual addr at execution
            sh_addr: Option<VirtualAddress>,

            /// Section file offset
            sh_offset: Option<FileOffset>,

            /// Section size in bytes
            sh_size: Option<Size>,

            /// Link to another section
            sh_link: Option<Link>,

            /// Additional section information
            sh_info: Option<Info>,

            /// Section alignment
            sh_addralign: Option<Align>,

            /// Entry size if section holds table
            sh_entsize: Option<EntrySize>,
        },
        display_implementation = true
    );
}
