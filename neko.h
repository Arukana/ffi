#ifndef NEKO_H
#define NEKO_H

/// Constants
#define SPEC_MAX_DRAW 16
#define SPEC_MAX_X 10
#define SPEC_MAX_Y 5
#define SPEC_MAX_XY SPEC_MAX_X*SPEC_MAX_Y
#define SPEC_CHARACTER_MAX 1024

/// Character Color Attributes
#define BLACK [ 0, 0, 0 ]
#define RED [ 255, 0, 0 ]
#define YELLOW [ 255, 255, 0 ]
#define GREEN [ 0, 255, 0 ]
#define CYAN [ 0, 255, 255 ]
#define BLUE [ 0, 0, 255 ]
#define MAGENTA [ 255, 0, 255 ]
#define WHITE [ 255, 255, 255 ]
#define DEFAULT_FOREGROUND BLACK
#define DEFAULT_BACKGROUND WHITE

/// Characters Style Attributes
typedef enum e_attribute {
	None_Attr = 0x00,
	Bold = 0x01,
	Dim = 0x02,
	Italic = 0x04,
	Underline = 0x08,
	Blink = 0x10,
	Reverse = 0x20,
	Hidden = 0x40,
} t_attribute;

/// Neko Default Placements into the display screen
typedef enum e_cardinal {
	UpperLeft,
	UpperMiddle,
	UpperRight,
	MiddleLeft,
	MiddleCentral,
	MiddleRight,
	LowerLeft,
	LowerMiddle,
	LowerRight,
} t_cardinal;

/// Neko's Postures
typedef enum e_sheet {
	None_Sheet = 95,
	Bust = 98,
} t_sheet;

/// Neko's Body Parts and Accessories
typedef enum e_part {
	None_Part = 95,
	ArmLeft = 97,
	ArmRight = 65,
	Boobs = 44,
	Clavicle = 99,
	EarLeft = 101,
	EarRight = 69,
	EyeLeft = 121,
	EyeRight = 89,
	HairTop = 111,
	HairLeft = 114,
	HairRight = 82,
	HandLeft = 100,
	HandRight = 68,
	Mouth = 109,
	Tail = 116,
	Bell = 108,
	ExclamationMark = 120,
	ExclamationMarks = 88,
	Heart = 104,
	Hearts = 72,
	Lantern = 110,
	QuestionMark = 113,
	QuestionMarks = 81,
	WoolBall = 119,
} t_part;

/// Neko's Emotions
typedef enum e_emotion {
	None_Emotion = 95,
	Angry = 97,
	Happy = 104,
	Love = 108,
	Malicious = 109,
	Misunderstanding = 105,
	Normal = 110,
	Playing = 112,
	Shocked = 111,
	Sleepy = 115,
	Speechless = 101,
	Surprised = 117,
} t_emotion;

typedef enum e_relative {
	Top = 0,
	Bottom = 1,
	Right = 2,
	Left = 3,
} t_relative;

/// Neko Placement Selection
typedef struct s_position {
	t_cardinal cardinal;
	unsigned short cartesian[2];
} t_position;

/// Neko's Texels Definition
typedef struct s_tuple {
	t_part part;
	t_emotion emotion;
} t_tuple;

/// Display screen's Characters
typedef struct s_character {
	/// Text Color
	unsigned char foreground[3];
	/// Background Color
	unsigned char background[3];
	/// Style Attributes
	unsigned char attribute;
	/// Glyph (as unicode character)
	unsigned int glyph;
} t_character;

typedef struct s_winszed {
    unsigned short ws_row;
    unsigned short ws_col;
    unsigned short ws_xpixel;
    unsigned short ws_ypixel;
} t_winszed;

typedef struct s_persona {
	t_sheet sheet;
	t_tuple expression[SPEC_MAX_DRAW][SPEC_MAX_XY];
	t_position position;
} t_persona;

typedef struct s_tooltip {
	t_relative relative;
	t_character message[SPEC_CHARACTER_MAX];
} t_tooltip;

typedef struct s_library_state {
	t_persona persona;
	t_tooltip tooltip;
	unsigned char unmount;
	unsigned char lock;
} t_lbstat;

#endif
