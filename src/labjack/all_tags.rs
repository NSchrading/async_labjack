use crate::prelude::*;
use bytes::Bytes;

/// Returns the voltage of the specified analog input.
pub const AIN0: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(0);
/// Returns the voltage of the specified analog input.
pub const AIN1: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(2);
/// Returns the voltage of the specified analog input.
pub const AIN2: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(4);
/// Returns the voltage of the specified analog input.
pub const AIN3: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(6);
/// Returns the voltage of the specified analog input.
pub const AIN4: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8);
/// Returns the voltage of the specified analog input.
pub const AIN5: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(10);
/// Returns the voltage of the specified analog input.
pub const AIN6: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(12);
/// Returns the voltage of the specified analog input.
pub const AIN7: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(14);
/// Returns the voltage of the specified analog input.
pub const AIN8: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(16);
/// Returns the voltage of the specified analog input.
pub const AIN9: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(18);
/// Returns the voltage of the specified analog input.
pub const AIN10: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(20);
/// Returns the voltage of the specified analog input.
pub const AIN11: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(22);
/// Returns the voltage of the specified analog input.
pub const AIN12: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(24);
/// Returns the voltage of the specified analog input.
pub const AIN13: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(26);
/// Returns the voltage of the specified analog input.
pub const AIN14: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(28);
/// Returns the voltage of the specified analog input.
pub const AIN15: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30);
/// Returns the voltage of the specified analog input.
pub const AIN16: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(32);
/// Returns the voltage of the specified analog input.
pub const AIN17: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(34);
/// Returns the voltage of the specified analog input.
pub const AIN18: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(36);
/// Returns the voltage of the specified analog input.
pub const AIN19: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(38);
/// Returns the voltage of the specified analog input.
pub const AIN20: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(40);
/// Returns the voltage of the specified analog input.
pub const AIN21: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(42);
/// Returns the voltage of the specified analog input.
pub const AIN22: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(44);
/// Returns the voltage of the specified analog input.
pub const AIN23: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(46);
/// Returns the voltage of the specified analog input.
pub const AIN24: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(48);
/// Returns the voltage of the specified analog input.
pub const AIN25: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(50);
/// Returns the voltage of the specified analog input.
pub const AIN26: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(52);
/// Returns the voltage of the specified analog input.
pub const AIN27: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(54);
/// Returns the voltage of the specified analog input.
pub const AIN28: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(56);
/// Returns the voltage of the specified analog input.
pub const AIN29: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(58);
/// Returns the voltage of the specified analog input.
pub const AIN30: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60);
/// Returns the voltage of the specified analog input.
pub const AIN31: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(62);
/// Returns the voltage of the specified analog input.
pub const AIN32: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(64);
/// Returns the voltage of the specified analog input.
pub const AIN33: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(66);
/// Returns the voltage of the specified analog input.
pub const AIN34: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(68);
/// Returns the voltage of the specified analog input.
pub const AIN35: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(70);
/// Returns the voltage of the specified analog input.
pub const AIN36: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(72);
/// Returns the voltage of the specified analog input.
pub const AIN37: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(74);
/// Returns the voltage of the specified analog input.
pub const AIN38: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(76);
/// Returns the voltage of the specified analog input.
pub const AIN39: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(78);
/// Returns the voltage of the specified analog input.
pub const AIN40: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(80);
/// Returns the voltage of the specified analog input.
pub const AIN41: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(82);
/// Returns the voltage of the specified analog input.
pub const AIN42: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(84);
/// Returns the voltage of the specified analog input.
pub const AIN43: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(86);
/// Returns the voltage of the specified analog input.
pub const AIN44: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(88);
/// Returns the voltage of the specified analog input.
pub const AIN45: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(90);
/// Returns the voltage of the specified analog input.
pub const AIN46: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(92);
/// Returns the voltage of the specified analog input.
pub const AIN47: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(94);
/// Returns the voltage of the specified analog input.
pub const AIN48: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(96);
/// Returns the voltage of the specified analog input.
pub const AIN49: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(98);
/// Returns the voltage of the specified analog input.
pub const AIN50: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(100);
/// Returns the voltage of the specified analog input.
pub const AIN51: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(102);
/// Returns the voltage of the specified analog input.
pub const AIN52: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(104);
/// Returns the voltage of the specified analog input.
pub const AIN53: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(106);
/// Returns the voltage of the specified analog input.
pub const AIN54: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(108);
/// Returns the voltage of the specified analog input.
pub const AIN55: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(110);
/// Returns the voltage of the specified analog input.
pub const AIN56: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(112);
/// Returns the voltage of the specified analog input.
pub const AIN57: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(114);
/// Returns the voltage of the specified analog input.
pub const AIN58: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(116);
/// Returns the voltage of the specified analog input.
pub const AIN59: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(118);
/// Returns the voltage of the specified analog input.
pub const AIN60: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(120);
/// Returns the voltage of the specified analog input.
pub const AIN61: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(122);
/// Returns the voltage of the specified analog input.
pub const AIN62: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(124);
/// Returns the voltage of the specified analog input.
pub const AIN63: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(126);
/// Returns the voltage of the specified analog input.
pub const AIN64: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(128);
/// Returns the voltage of the specified analog input.
pub const AIN65: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(130);
/// Returns the voltage of the specified analog input.
pub const AIN66: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(132);
/// Returns the voltage of the specified analog input.
pub const AIN67: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(134);
/// Returns the voltage of the specified analog input.
pub const AIN68: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(136);
/// Returns the voltage of the specified analog input.
pub const AIN69: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(138);
/// Returns the voltage of the specified analog input.
pub const AIN70: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(140);
/// Returns the voltage of the specified analog input.
pub const AIN71: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(142);
/// Returns the voltage of the specified analog input.
pub const AIN72: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(144);
/// Returns the voltage of the specified analog input.
pub const AIN73: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(146);
/// Returns the voltage of the specified analog input.
pub const AIN74: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(148);
/// Returns the voltage of the specified analog input.
pub const AIN75: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(150);
/// Returns the voltage of the specified analog input.
pub const AIN76: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(152);
/// Returns the voltage of the specified analog input.
pub const AIN77: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(154);
/// Returns the voltage of the specified analog input.
pub const AIN78: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(156);
/// Returns the voltage of the specified analog input.
pub const AIN79: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(158);
/// Returns the voltage of the specified analog input.
pub const AIN80: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(160);
/// Returns the voltage of the specified analog input.
pub const AIN81: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(162);
/// Returns the voltage of the specified analog input.
pub const AIN82: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(164);
/// Returns the voltage of the specified analog input.
pub const AIN83: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(166);
/// Returns the voltage of the specified analog input.
pub const AIN84: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(168);
/// Returns the voltage of the specified analog input.
pub const AIN85: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(170);
/// Returns the voltage of the specified analog input.
pub const AIN86: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(172);
/// Returns the voltage of the specified analog input.
pub const AIN87: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(174);
/// Returns the voltage of the specified analog input.
pub const AIN88: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(176);
/// Returns the voltage of the specified analog input.
pub const AIN89: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(178);
/// Returns the voltage of the specified analog input.
pub const AIN90: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(180);
/// Returns the voltage of the specified analog input.
pub const AIN91: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(182);
/// Returns the voltage of the specified analog input.
pub const AIN92: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(184);
/// Returns the voltage of the specified analog input.
pub const AIN93: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(186);
/// Returns the voltage of the specified analog input.
pub const AIN94: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(188);
/// Returns the voltage of the specified analog input.
pub const AIN95: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(190);
/// Returns the voltage of the specified analog input.
pub const AIN96: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(192);
/// Returns the voltage of the specified analog input.
pub const AIN97: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(194);
/// Returns the voltage of the specified analog input.
pub const AIN98: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(196);
/// Returns the voltage of the specified analog input.
pub const AIN99: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(198);
/// Returns the voltage of the specified analog input.
pub const AIN100: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(200);
/// Returns the voltage of the specified analog input.
pub const AIN101: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(202);
/// Returns the voltage of the specified analog input.
pub const AIN102: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(204);
/// Returns the voltage of the specified analog input.
pub const AIN103: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(206);
/// Returns the voltage of the specified analog input.
pub const AIN104: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(208);
/// Returns the voltage of the specified analog input.
pub const AIN105: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(210);
/// Returns the voltage of the specified analog input.
pub const AIN106: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(212);
/// Returns the voltage of the specified analog input.
pub const AIN107: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(214);
/// Returns the voltage of the specified analog input.
pub const AIN108: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(216);
/// Returns the voltage of the specified analog input.
pub const AIN109: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(218);
/// Returns the voltage of the specified analog input.
pub const AIN110: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(220);
/// Returns the voltage of the specified analog input.
pub const AIN111: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(222);
/// Returns the voltage of the specified analog input.
pub const AIN112: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(224);
/// Returns the voltage of the specified analog input.
pub const AIN113: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(226);
/// Returns the voltage of the specified analog input.
pub const AIN114: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(228);
/// Returns the voltage of the specified analog input.
pub const AIN115: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(230);
/// Returns the voltage of the specified analog input.
pub const AIN116: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(232);
/// Returns the voltage of the specified analog input.
pub const AIN117: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(234);
/// Returns the voltage of the specified analog input.
pub const AIN118: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(236);
/// Returns the voltage of the specified analog input.
pub const AIN119: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(238);
/// Returns the voltage of the specified analog input.
pub const AIN120: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(240);
/// Returns the voltage of the specified analog input.
pub const AIN121: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(242);
/// Returns the voltage of the specified analog input.
pub const AIN122: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(244);
/// Returns the voltage of the specified analog input.
pub const AIN123: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(246);
/// Returns the voltage of the specified analog input.
pub const AIN124: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(248);
/// Returns the voltage of the specified analog input.
pub const AIN125: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(250);
/// Returns the voltage of the specified analog input.
pub const AIN126: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(252);
/// Returns the voltage of the specified analog input.
pub const AIN127: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(254);
/// Returns the voltage of the specified analog input.
pub const AIN128: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(256);
/// Returns the voltage of the specified analog input.
pub const AIN129: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(258);
/// Returns the voltage of the specified analog input.
pub const AIN130: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(260);
/// Returns the voltage of the specified analog input.
pub const AIN131: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(262);
/// Returns the voltage of the specified analog input.
pub const AIN132: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(264);
/// Returns the voltage of the specified analog input.
pub const AIN133: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(266);
/// Returns the voltage of the specified analog input.
pub const AIN134: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(268);
/// Returns the voltage of the specified analog input.
pub const AIN135: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(270);
/// Returns the voltage of the specified analog input.
pub const AIN136: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(272);
/// Returns the voltage of the specified analog input.
pub const AIN137: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(274);
/// Returns the voltage of the specified analog input.
pub const AIN138: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(276);
/// Returns the voltage of the specified analog input.
pub const AIN139: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(278);
/// Returns the voltage of the specified analog input.
pub const AIN140: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(280);
/// Returns the voltage of the specified analog input.
pub const AIN141: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(282);
/// Returns the voltage of the specified analog input.
pub const AIN142: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(284);
/// Returns the voltage of the specified analog input.
pub const AIN143: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(286);
/// Returns the voltage of the specified analog input.
pub const AIN144: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(288);
/// Returns the voltage of the specified analog input.
pub const AIN145: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(290);
/// Returns the voltage of the specified analog input.
pub const AIN146: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(292);
/// Returns the voltage of the specified analog input.
pub const AIN147: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(294);
/// Returns the voltage of the specified analog input.
pub const AIN148: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(296);
/// Returns the voltage of the specified analog input.
pub const AIN149: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(298);
/// Returns the voltage of the specified analog input.
pub const AIN150: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(300);
/// Returns the voltage of the specified analog input.
pub const AIN151: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(302);
/// Returns the voltage of the specified analog input.
pub const AIN152: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(304);
/// Returns the voltage of the specified analog input.
pub const AIN153: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(306);
/// Returns the voltage of the specified analog input.
pub const AIN154: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(308);
/// Returns the voltage of the specified analog input.
pub const AIN155: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(310);
/// Returns the voltage of the specified analog input.
pub const AIN156: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(312);
/// Returns the voltage of the specified analog input.
pub const AIN157: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(314);
/// Returns the voltage of the specified analog input.
pub const AIN158: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(316);
/// Returns the voltage of the specified analog input.
pub const AIN159: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(318);
/// Returns the voltage of the specified analog input.
pub const AIN160: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(320);
/// Returns the voltage of the specified analog input.
pub const AIN161: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(322);
/// Returns the voltage of the specified analog input.
pub const AIN162: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(324);
/// Returns the voltage of the specified analog input.
pub const AIN163: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(326);
/// Returns the voltage of the specified analog input.
pub const AIN164: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(328);
/// Returns the voltage of the specified analog input.
pub const AIN165: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(330);
/// Returns the voltage of the specified analog input.
pub const AIN166: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(332);
/// Returns the voltage of the specified analog input.
pub const AIN167: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(334);
/// Returns the voltage of the specified analog input.
pub const AIN168: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(336);
/// Returns the voltage of the specified analog input.
pub const AIN169: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(338);
/// Returns the voltage of the specified analog input.
pub const AIN170: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(340);
/// Returns the voltage of the specified analog input.
pub const AIN171: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(342);
/// Returns the voltage of the specified analog input.
pub const AIN172: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(344);
/// Returns the voltage of the specified analog input.
pub const AIN173: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(346);
/// Returns the voltage of the specified analog input.
pub const AIN174: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(348);
/// Returns the voltage of the specified analog input.
pub const AIN175: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(350);
/// Returns the voltage of the specified analog input.
pub const AIN176: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(352);
/// Returns the voltage of the specified analog input.
pub const AIN177: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(354);
/// Returns the voltage of the specified analog input.
pub const AIN178: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(356);
/// Returns the voltage of the specified analog input.
pub const AIN179: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(358);
/// Returns the voltage of the specified analog input.
pub const AIN180: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(360);
/// Returns the voltage of the specified analog input.
pub const AIN181: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(362);
/// Returns the voltage of the specified analog input.
pub const AIN182: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(364);
/// Returns the voltage of the specified analog input.
pub const AIN183: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(366);
/// Returns the voltage of the specified analog input.
pub const AIN184: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(368);
/// Returns the voltage of the specified analog input.
pub const AIN185: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(370);
/// Returns the voltage of the specified analog input.
pub const AIN186: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(372);
/// Returns the voltage of the specified analog input.
pub const AIN187: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(374);
/// Returns the voltage of the specified analog input.
pub const AIN188: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(376);
/// Returns the voltage of the specified analog input.
pub const AIN189: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(378);
/// Returns the voltage of the specified analog input.
pub const AIN190: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(380);
/// Returns the voltage of the specified analog input.
pub const AIN191: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(382);
/// Returns the voltage of the specified analog input.
pub const AIN192: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(384);
/// Returns the voltage of the specified analog input.
pub const AIN193: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(386);
/// Returns the voltage of the specified analog input.
pub const AIN194: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(388);
/// Returns the voltage of the specified analog input.
pub const AIN195: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(390);
/// Returns the voltage of the specified analog input.
pub const AIN196: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(392);
/// Returns the voltage of the specified analog input.
pub const AIN197: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(394);
/// Returns the voltage of the specified analog input.
pub const AIN198: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(396);
/// Returns the voltage of the specified analog input.
pub const AIN199: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(398);
/// Returns the voltage of the specified analog input.
pub const AIN200: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(400);
/// Returns the voltage of the specified analog input.
pub const AIN201: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(402);
/// Returns the voltage of the specified analog input.
pub const AIN202: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(404);
/// Returns the voltage of the specified analog input.
pub const AIN203: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(406);
/// Returns the voltage of the specified analog input.
pub const AIN204: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(408);
/// Returns the voltage of the specified analog input.
pub const AIN205: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(410);
/// Returns the voltage of the specified analog input.
pub const AIN206: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(412);
/// Returns the voltage of the specified analog input.
pub const AIN207: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(414);
/// Returns the voltage of the specified analog input.
pub const AIN208: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(416);
/// Returns the voltage of the specified analog input.
pub const AIN209: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(418);
/// Returns the voltage of the specified analog input.
pub const AIN210: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(420);
/// Returns the voltage of the specified analog input.
pub const AIN211: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(422);
/// Returns the voltage of the specified analog input.
pub const AIN212: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(424);
/// Returns the voltage of the specified analog input.
pub const AIN213: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(426);
/// Returns the voltage of the specified analog input.
pub const AIN214: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(428);
/// Returns the voltage of the specified analog input.
pub const AIN215: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(430);
/// Returns the voltage of the specified analog input.
pub const AIN216: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(432);
/// Returns the voltage of the specified analog input.
pub const AIN217: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(434);
/// Returns the voltage of the specified analog input.
pub const AIN218: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(436);
/// Returns the voltage of the specified analog input.
pub const AIN219: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(438);
/// Returns the voltage of the specified analog input.
pub const AIN220: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(440);
/// Returns the voltage of the specified analog input.
pub const AIN221: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(442);
/// Returns the voltage of the specified analog input.
pub const AIN222: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(444);
/// Returns the voltage of the specified analog input.
pub const AIN223: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(446);
/// Returns the voltage of the specified analog input.
pub const AIN224: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(448);
/// Returns the voltage of the specified analog input.
pub const AIN225: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(450);
/// Returns the voltage of the specified analog input.
pub const AIN226: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(452);
/// Returns the voltage of the specified analog input.
pub const AIN227: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(454);
/// Returns the voltage of the specified analog input.
pub const AIN228: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(456);
/// Returns the voltage of the specified analog input.
pub const AIN229: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(458);
/// Returns the voltage of the specified analog input.
pub const AIN230: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(460);
/// Returns the voltage of the specified analog input.
pub const AIN231: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(462);
/// Returns the voltage of the specified analog input.
pub const AIN232: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(464);
/// Returns the voltage of the specified analog input.
pub const AIN233: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(466);
/// Returns the voltage of the specified analog input.
pub const AIN234: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(468);
/// Returns the voltage of the specified analog input.
pub const AIN235: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(470);
/// Returns the voltage of the specified analog input.
pub const AIN236: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(472);
/// Returns the voltage of the specified analog input.
pub const AIN237: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(474);
/// Returns the voltage of the specified analog input.
pub const AIN238: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(476);
/// Returns the voltage of the specified analog input.
pub const AIN239: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(478);
/// Returns the voltage of the specified analog input.
pub const AIN240: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(480);
/// Returns the voltage of the specified analog input.
pub const AIN241: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(482);
/// Returns the voltage of the specified analog input.
pub const AIN242: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(484);
/// Returns the voltage of the specified analog input.
pub const AIN243: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(486);
/// Returns the voltage of the specified analog input.
pub const AIN244: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(488);
/// Returns the voltage of the specified analog input.
pub const AIN245: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(490);
/// Returns the voltage of the specified analog input.
pub const AIN246: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(492);
/// Returns the voltage of the specified analog input.
pub const AIN247: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(494);
/// Returns the voltage of the specified analog input.
pub const AIN248: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(496);
/// Returns the voltage of the specified analog input.
pub const AIN249: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(498);
/// Returns the voltage of the specified analog input.
pub const AIN250: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(500);
/// Returns the voltage of the specified analog input.
pub const AIN251: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(502);
/// Returns the voltage of the specified analog input.
pub const AIN252: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(504);
/// Returns the voltage of the specified analog input.
pub const AIN253: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(506);
/// Returns the voltage of the specified analog input.
pub const AIN254: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(508);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE0: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(600);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE1: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(602);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE2: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(604);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE3: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(606);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE4: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(608);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE5: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(610);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE6: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(612);
/// T8 Only. Returns the temperature of the specified analog input. And saves AIN and temperature inputs for all channels.
pub const TEMPERATURE7: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(614);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN0_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(650);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN1_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(652);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN2_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(654);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN3_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(656);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN4_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(658);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN5_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(660);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN6_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(662);
/// T8 Only. Returns the saved voltage of the specified analog input.
pub const AIN7_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(664);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE0_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(700);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE1_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(702);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE2_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(704);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE3_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(706);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE4_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(708);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE5_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(710);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE6_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(712);
/// T8 Only. Returns the saved temperature of the specified analog input.
pub const TEMPERATURE7_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(714);
/// Pass a voltage for the specified analog output.
pub const DAC0: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(1000);
/// Pass a voltage for the specified analog output.
pub const DAC1: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(1002);
/// Fixed current source value in Amps for the 10UA terminal. This value is stored during factory calibration, it is not a current reading. Using the equation V=IR, with a known current and voltage, it is possible to calculate resistance of RTDs.
pub const CURRENT_SOURCE_10UA_CAL_VALUE: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(1900);
/// Fixed current source value in Amps for the 200UA terminal. This value is stored during factory calibration, it is not a current reading. Using the equation V=IR, with a known current and voltage, it is possible to calculate resistance of RTDs.
pub const CURRENT_SOURCE_200UA_CAL_VALUE: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(1902);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO0: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2000);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO1: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2001);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO2: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2002);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO3: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2003);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO4: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2004);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO5: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2005);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO6: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2006);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const FIO7: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2007);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO0: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2008);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO1: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2009);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO2: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2010);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO3: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2011);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO4: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2012);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO5: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2013);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO6: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2014);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const EIO7: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2015);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const CIO0: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2016);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const CIO1: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2017);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const CIO2: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2018);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const CIO3: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2019);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const MIO0: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2020);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const MIO1: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2021);
/// Read or set the state of 1 bit of digital I/O.  Also configures the direction to input or output. Read 0=Low AND 1=High. Write 0=Low AND 1=High.
pub const MIO2: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2022);
/// Read or write the state of the 8 bits of FIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written.  The upper 8-bits of this value are inhibits.
pub const FIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2500);
/// Read or write the state of the 8 bits of EIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written.  The upper 8-bits of this value are inhibits.
pub const EIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2501);
/// Read or write the state of the 4 bits of CIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written.  The upper 8-bits of this value are inhibits.
pub const CIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2502);
/// Read or write the state of the 3 bits of MIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written.  The upper 8-bits of this value are inhibits.
pub const MIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2503);
/// Read or write the state of the 16 bits of FIO-EIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written.
pub const FIO_EIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2580);
/// Read or write the state of the 12 bits of EIO-CIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written. As of firmware 1.0172, MIO states are included in the upper nibble of the CIO byte.
pub const EIO_CIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2581);
/// Read or write the state of the 12 bits of CIO-MIO in a single binary-encoded value. 0=Low AND 1=High.  Does not configure direction.  Reading lines set to output returns the current logic levels on the terminals, not necessarily the output states written.
pub const CIO_MIO_STATE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2582);
/// Read or write the direction of the 8 bits of FIO in a single binary-encoded value.  0=Input and 1=Output.  The upper 8-bits of this value are inhibits.
pub const FIO_DIRECTION: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2600);
/// Read or write the direction of the 8 bits of EIO in a single binary-encoded value.  0=Input and 1=Output.  The upper 8-bits of this value are inhibits.
pub const EIO_DIRECTION: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2601);
/// Read or write the direction of the 4 bits of CIO in a single binary-encoded value.  0=Input and 1=Output.  The upper 8-bits of this value are inhibits.
pub const CIO_DIRECTION: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2602);
/// Read or write the direction of the 3 bits of MIO in a single binary-encoded value.  0=Input and 1=Output.  The upper 8-bits of this value are inhibits.
pub const MIO_DIRECTION: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(2603);
/// Read or write the state of all digital I/O in a single binary-encoded value. 0=Low AND 1=High. Does not configure direction.  A read of an output returns the current logic level on the terminal, not necessarily the output state written.  Writes are filtered by the value in DIO_INHIBIT.
pub const DIO_STATE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(2800);
/// Read or write the direction of all digital I/O in a single binary-encoded value.  0=Input and 1=Output.  Writes are filtered by the value in DIO_INHIBIT.
pub const DIO_DIRECTION: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(2850);
/// T8 Only. This register will enable pulldowns on DIO lines. This is a binary coded value where bit 0 represent FIO0, and bit 11 represents EIO3, etc. 1 = pulldown enabled, 0 = pulldown disabled. This register only affects flex-lines which can be configured as analog or digital. This register is not affected by the inhibit register.
pub const DIO_PULLDOWN_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(2870);
/// Read or write the analog configuration of all digital I/O in a single binary-encoded value.  1=Analog mode and 0=Digital mode. When switching from analog to digital, the lines will be set to input. Writes are filtered by the value in DIO_INHIBIT.
pub const DIO_ANALOG_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(2880);
/// This register will prevent pullups from being enabled on lines set to digital input. This is a binary coded value where bit 0 represent FIO0 and bit 11 represents EIO3. 1 = pullup disabled, 0 = pullup enabled. This register is not affected by the inhibit register.
pub const DIO_PULLUP_DISABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(2890);
/// A single binary-encoded value where each bit determines whether _STATE, _DIRECTION or _ANALOG_ENABLE writes affect that bit of digital I/O. 0=Default=Affected, 1=Ignored.
pub const DIO_INHIBIT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(2900);
/// Sets the state of the COMM LED when the LEDs are set to manual, see the POWER_LED register.
pub const LED_COMM: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(2990);
/// Sets the state of the STATUS LED when the LEDs are set to manual, see the POWER_LED register.
pub const LED_STATUS: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(2991);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO0_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3000);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO1_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3002);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO2_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3004);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO3_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3006);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO4_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3008);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO5_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3010);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO6_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3012);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO7_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3014);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO8_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3016);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO9_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3018);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO10_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3020);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO11_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3022);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO12_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3024);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO13_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3026);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO14_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3028);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO15_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3030);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO16_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3032);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO17_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3034);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO18_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3036);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO19_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3038);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO20_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3040);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO21_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3042);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO22_EF_READ_A: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3044);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO0_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3100);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO1_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3102);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO2_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3104);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO3_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3106);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO4_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3108);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO5_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3110);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO6_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3112);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO7_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3114);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO8_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3116);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO9_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3118);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO10_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3120);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO11_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3122);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO12_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3124);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO13_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3126);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO14_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3128);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO15_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3130);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO16_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3132);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO17_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3134);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO18_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3136);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO19_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3138);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO20_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3140);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO21_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3142);
/// Reads the same value as DIO#(0:22)_EF_READ_A and forces a reset.
pub const DIO22_EF_READ_A_AND_RESET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3144);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO0_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3200);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO1_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3202);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO2_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3204);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO3_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3206);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO4_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3208);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO5_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3210);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO6_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3212);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO7_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3214);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO8_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3216);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO9_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3218);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO10_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3220);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO11_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3222);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO12_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3224);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO13_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3226);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO14_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3228);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO15_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3230);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO16_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3232);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO17_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3234);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO18_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3236);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO19_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3238);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO20_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3240);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO21_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3242);
/// Reads an unsigned integer value. The meaning of the integer is dependent on selected feature index.
pub const DIO22_EF_READ_B: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(3244);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO0_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3500);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO1_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3502);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO2_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3504);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO3_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3506);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO4_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3508);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO5_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3510);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO6_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3512);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO7_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3514);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO8_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3516);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO9_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3518);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO10_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3520);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO11_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3522);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO12_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3524);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO13_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3526);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO14_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3528);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO15_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3530);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO16_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3532);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO17_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3534);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO18_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3536);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO19_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3538);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO20_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3540);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO21_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3542);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO22_EF_READ_A_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3544);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO0_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3600);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO1_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3602);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO2_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3604);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO3_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3606);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO4_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3608);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO5_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3610);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO6_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3612);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO7_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3614);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO8_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3616);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO9_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3618);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO10_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3620);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO11_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3622);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO12_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3624);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO13_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3626);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO14_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3628);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO15_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3630);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO16_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3632);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO17_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3634);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO18_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3636);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO19_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3638);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO20_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3640);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO21_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3642);
/// Reads a floating point value and forces a reset. The meaning of value is dependent on selected feature index.
pub const DIO22_EF_READ_A_F_AND_RESET: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(3644);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO0_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3700);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO1_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3702);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO2_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3704);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO3_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3706);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO4_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3708);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO5_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3710);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO6_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3712);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO7_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3714);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO8_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3716);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO9_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3718);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO10_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3720);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO11_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3722);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO12_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3724);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO13_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3726);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO14_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3728);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO15_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3730);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO16_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3732);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO17_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3734);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO18_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3736);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO19_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3738);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO20_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3740);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO21_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3742);
/// Reads a floating point value. The meaning of value is dependent on selected feature index.
pub const DIO22_EF_READ_B_F: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(3744);
/// Write a value to specify the number of times per second that all channels in the stream scanlist will be read.  Max stream speeds are based on Sample Rate which is NumChannels*ScanRate.  Has no effect when using an external clock.  A read of this register returns the actual scan rate, which can be slightly different due to rounding.  For scan rates >152.588, the actual scan interval is multiples of 100 ns.  Assuming a core clock of 80 MHz the internal roll value is (80M/(8*DesiredScanRate))-1 and the actual scan rate is then 80M/(8*(RollValue+1).  For slower scan rates the scan interval resolution is changed to 1 us, 10 us, 100 us, or 1 ms as needed to achieve the longer intervals.
pub const STREAM_SCANRATE_HZ: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(4002);
/// The number of entries in the scanlist
pub const STREAM_NUM_ADDRESSES: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4004);
/// Specifies the number of data points to be sent in the data packet. Only applies to spontaneous mode.
pub const STREAM_SAMPLES_PER_PACKET: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4006);
/// Time in microseconds to allow signals to settle after switching the mux. Does not apply to the 1st channel in the scan list, as that settling is controlled by scan rate (the time from the last channel until the start of the next scan). Default=0. When set to less than 1, automatic settling will be used. The automatic settling behavior varies by device.
pub const STREAM_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(4008);
/// The resolution index for stream readings. A larger resolution index generally results in lower noise and longer sample times.
pub const STREAM_RESOLUTION_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4010);
/// Size of the stream data buffer in bytes. A value of 0 equates to the default value.  Must be a power of 2. Size in samples is STREAM_BUFFER_SIZE_BYTES/2.  Size in scans is (STREAM_BUFFER_SIZE_BYTES/2)/STREAM_NUM_ADDRESSES. Changes while stream is running do not affect the currently running stream.
pub const STREAM_BUFFER_SIZE_BYTES: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4012);
/// Controls which clock source will be used to run the main stream clock. 0 = Internal crystal, 2 = External clock, 4 = Primary ESS, 8 = Secondary ESS
pub const STREAM_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4014);
/// Controls where data will be sent. Value is a bitmask. bit 0: 1 = Send to Ethernet 702 sockets, bit 1: 1 = Send to USB, bit 4: 1 = Command-Response mode. All other bits are reserved.
pub const STREAM_AUTO_TARGET: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4016);
/// Must be written with the value 0 for streaming.
pub const STREAM_DATATYPE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4018);
/// The number of scans to run before automatically stopping (stream-burst). 0 = run continuously. Limit for STREAM_NUM_SCANS is 2^32-1, but if the host is not reading data as fast as it is acquired you also need to consider STREAM_BUFFER_SIZE_BYTES.
pub const STREAM_NUM_SCANS: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4020);
/// The number of pulses per stream scan when using an external clock.
pub const STREAM_EXTERNAL_CLOCK_DIVISOR: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4022);
/// Controls when stream scanning will start. 0 = Start when stream is enabled, 2000 = Start when DIO_EF0 detects an edge, 2001 = Start when DIO_EF1 detects an edge. See the stream documentation for all supported values.
pub const STREAM_TRIGGER_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4024);
/// This register stores the value of CORE_TIMER at the start of the first stream scan.  Note that the first stream scan happens 1 scan period after STREAM_ENABLE is set to 1.
pub const STREAM_START_TIME_STAMP: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(4026);
/// When the stream buffer becomes full the stream will be stopped. Same behavior as if STREAM_ENABLE is set to zero.
pub const STREAM_AUTORECOVER_DISABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4028);
/// Channel that data will be written to.  Before writing data to _BUFFER_###, you must write to _TARGET so the device knows how to interpret and store values.
pub const STREAM_OUT0_TARGET: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4040);
/// Channel that data will be written to.  Before writing data to _BUFFER_###, you must write to _TARGET so the device knows how to interpret and store values.
pub const STREAM_OUT1_TARGET: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4042);
/// Channel that data will be written to.  Before writing data to _BUFFER_###, you must write to _TARGET so the device knows how to interpret and store values.
pub const STREAM_OUT2_TARGET: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4044);
/// Channel that data will be written to.  Before writing data to _BUFFER_###, you must write to _TARGET so the device knows how to interpret and store values.
pub const STREAM_OUT3_TARGET: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4046);
/// Size of the buffer in bytes as a power of 2. Should be at least twice the size of updates that will be written and no less than 32. The usable buffer size will be equal to the value of this register minus 2 bytes (one 16-bit sample). Before writing data to _BUFFER_###, you must write to _BUFFER_ALLOCATE_NUM_BYTES to allocate RAM for the data. Max is 16384.
pub const STREAM_OUT0_BUFFER_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(4050);
/// Size of the buffer in bytes as a power of 2. Should be at least twice the size of updates that will be written and no less than 32. The usable buffer size will be equal to the value of this register minus 2 bytes (one 16-bit sample). Before writing data to _BUFFER_###, you must write to _BUFFER_ALLOCATE_NUM_BYTES to allocate RAM for the data. Max is 16384.
pub const STREAM_OUT1_BUFFER_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(4052);
/// Size of the buffer in bytes as a power of 2. Should be at least twice the size of updates that will be written and no less than 32. The usable buffer size will be equal to the value of this register minus 2 bytes (one 16-bit sample). Before writing data to _BUFFER_###, you must write to _BUFFER_ALLOCATE_NUM_BYTES to allocate RAM for the data. Max is 16384.
pub const STREAM_OUT2_BUFFER_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(4054);
/// Size of the buffer in bytes as a power of 2. Should be at least twice the size of updates that will be written and no less than 32. The usable buffer size will be equal to the value of this register minus 2 bytes (one 16-bit sample). Before writing data to _BUFFER_###, you must write to _BUFFER_ALLOCATE_NUM_BYTES to allocate RAM for the data. Max is 16384.
pub const STREAM_OUT3_BUFFER_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(4056);
/// The number of values, from the end of the array, that will be repeated after reaching the end of supplied data array.
pub const STREAM_OUT0_LOOP_NUM_VALUES: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4060);
/// The number of values, from the end of the array, that will be repeated after reaching the end of supplied data array.
pub const STREAM_OUT1_LOOP_NUM_VALUES: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4062);
/// The number of values, from the end of the array, that will be repeated after reaching the end of supplied data array.
pub const STREAM_OUT2_LOOP_NUM_VALUES: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4064);
/// The number of values, from the end of the array, that will be repeated after reaching the end of supplied data array.
pub const STREAM_OUT3_LOOP_NUM_VALUES: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4066);
/// Controls when new data and loop size are used. 1=Use new data immediately. 2=Wait for synch. New data will not be used until a different stream-out channel is set to Synch. 3=Synch. This stream-out# as well as any stream-outs set to synch will start using new data immediately.
pub const STREAM_OUT0_SET_LOOP: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4070);
/// Controls when new data and loop size are used. 1=Use new data immediately. 2=Wait for synch. New data will not be used until a different stream-out channel is set to Synch. 3=Synch. This stream-out# as well as any stream-outs set to synch will start using new data immediately.
pub const STREAM_OUT1_SET_LOOP: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4072);
/// Controls when new data and loop size are used. 1=Use new data immediately. 2=Wait for synch. New data will not be used until a different stream-out channel is set to Synch. 3=Synch. This stream-out# as well as any stream-outs set to synch will start using new data immediately.
pub const STREAM_OUT2_SET_LOOP: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4074);
/// Controls when new data and loop size are used. 1=Use new data immediately. 2=Wait for synch. New data will not be used until a different stream-out channel is set to Synch. 3=Synch. This stream-out# as well as any stream-outs set to synch will start using new data immediately.
pub const STREAM_OUT3_SET_LOOP: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4076);
/// The number of bytes left in the buffer, where each sample is two bytes. Used to monitor where the stream out is in processing the buffer.
pub const STREAM_OUT0_BUFFER_STATUS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(4080);
/// The number of bytes left in the buffer, where each sample is two bytes. Used to monitor where the stream out is in processing the buffer.
pub const STREAM_OUT1_BUFFER_STATUS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(4082);
/// The number of bytes left in the buffer, where each sample is two bytes. Used to monitor where the stream out is in processing the buffer.
pub const STREAM_OUT2_BUFFER_STATUS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(4084);
/// The number of bytes left in the buffer, where each sample is two bytes. Used to monitor where the stream out is in processing the buffer.
pub const STREAM_OUT3_BUFFER_STATUS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(4086);
/// When STREAM_OUT#_ENABLE is enabled, the stream out target is generally updated by one value from the stream out buffer per stream scan. For example, there will generally be one instance of e.g. STREAM_OUT0 in the stream scan list, which will cause one STREAM_OUT0_BUFFER value to be consumed and written to STREAM_OUT0_TARGET for every stream scan. The stream scan list could also contain two instances of STREAM_OUT0, in which case two values from STREAM_OUT0_BUFFER value would be consumed and written for every stream scan.
pub const STREAM_OUT0_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4090);
/// When STREAM_OUT#_ENABLE is enabled, the stream out target is generally updated by one value from the stream out buffer per stream scan. For example, there will generally be one instance of e.g. STREAM_OUT0 in the stream scan list, which will cause one STREAM_OUT0_BUFFER value to be consumed and written to STREAM_OUT0_TARGET for every stream scan. The stream scan list could also contain two instances of STREAM_OUT0, in which case two values from STREAM_OUT0_BUFFER value would be consumed and written for every stream scan.
pub const STREAM_OUT1_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4092);
/// When STREAM_OUT#_ENABLE is enabled, the stream out target is generally updated by one value from the stream out buffer per stream scan. For example, there will generally be one instance of e.g. STREAM_OUT0 in the stream scan list, which will cause one STREAM_OUT0_BUFFER value to be consumed and written to STREAM_OUT0_TARGET for every stream scan. The stream scan list could also contain two instances of STREAM_OUT0, in which case two values from STREAM_OUT0_BUFFER value would be consumed and written for every stream scan.
pub const STREAM_OUT2_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4094);
/// When STREAM_OUT#_ENABLE is enabled, the stream out target is generally updated by one value from the stream out buffer per stream scan. For example, there will generally be one instance of e.g. STREAM_OUT0 in the stream scan list, which will cause one STREAM_OUT0_BUFFER value to be consumed and written to STREAM_OUT0_TARGET for every stream scan. The stream scan list could also contain two instances of STREAM_OUT0, in which case two values from STREAM_OUT0_BUFFER value would be consumed and written for every stream scan.
pub const STREAM_OUT3_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4096);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS0: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4100);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS1: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4102);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS2: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4104);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS3: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4106);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS4: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4108);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS5: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4110);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS6: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4112);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS7: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4114);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS8: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4116);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS9: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4118);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS10: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4120);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS11: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4122);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS12: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4124);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS13: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4126);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS14: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4128);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS15: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4130);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS16: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4132);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS17: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4134);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS18: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4136);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS19: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4138);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS20: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4140);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS21: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4142);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS22: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4144);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS23: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4146);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS24: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4148);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS25: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4150);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS26: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4152);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS27: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4154);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS28: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4156);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS29: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4158);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS30: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4160);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS31: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4162);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4164);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS33: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4166);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS34: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4168);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS35: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4170);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS36: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4172);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS37: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4174);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS38: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4176);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS39: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4178);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS40: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4180);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS41: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4182);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS42: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4184);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS43: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4186);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS44: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4188);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS45: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4190);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS46: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4192);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS47: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4194);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS48: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4196);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS49: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4198);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS50: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4200);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS51: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4202);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS52: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4204);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS53: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4206);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS54: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4208);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS55: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4210);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS56: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4212);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS57: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4214);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS58: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4216);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS59: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4218);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS60: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4220);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS61: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4222);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS62: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4224);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS63: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4226);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS64: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4228);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS65: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4230);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS66: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4232);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS67: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4234);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS68: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4236);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS69: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4238);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS70: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4240);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS71: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4242);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS72: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4244);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS73: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4246);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS74: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4248);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS75: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4250);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS76: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4252);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS77: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4254);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS78: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4256);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS79: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4258);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS80: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4260);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS81: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4262);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS82: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4264);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS83: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4266);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS84: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4268);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS85: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4270);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS86: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4272);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS87: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4274);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS88: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4276);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS89: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4278);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS90: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4280);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS91: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4282);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS92: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4284);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS93: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4286);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS94: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4288);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS95: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4290);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS96: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4292);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS97: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4294);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS98: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4296);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS99: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4298);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS100: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4300);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS101: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4302);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS102: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4304);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS103: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4306);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS104: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4308);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS105: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4310);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS106: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4312);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS107: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4314);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS108: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4316);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS109: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4318);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS110: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4320);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS111: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4322);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS112: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4324);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS113: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4326);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS114: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4328);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS115: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4330);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS116: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4332);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS117: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4334);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS118: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4336);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS119: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4338);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS120: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4340);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS121: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4342);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS122: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4344);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS123: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4346);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS124: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4348);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS125: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4350);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS126: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4352);
/// A list of addresses to read each scan. In the case of Stream-Out enabled, the list may also include something to write each scan.
pub const STREAM_SCANLIST_ADDRESS127: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4354);
/// Data destination when sending floating point data. Appropriate cal constants are used to convert F32 values to 16-bit binary data, and thus each of these values uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT0_BUFFER_F32: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(4400);
/// Data destination when sending floating point data. Appropriate cal constants are used to convert F32 values to 16-bit binary data, and thus each of these values uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT1_BUFFER_F32: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(4402);
/// Data destination when sending floating point data. Appropriate cal constants are used to convert F32 values to 16-bit binary data, and thus each of these values uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT2_BUFFER_F32: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(4404);
/// Data destination when sending floating point data. Appropriate cal constants are used to convert F32 values to 16-bit binary data, and thus each of these values uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT3_BUFFER_F32: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(4406);
/// Not used at this time.  There are no U32 registers supported in stream-out. This register is a buffer.
pub const STREAM_OUT0_BUFFER_U32: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4410);
/// Not used at this time.  There are no U32 registers supported in stream-out. This register is a buffer.
pub const STREAM_OUT1_BUFFER_U32: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4412);
/// Not used at this time.  There are no U32 registers supported in stream-out. This register is a buffer.
pub const STREAM_OUT2_BUFFER_U32: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4414);
/// Not used at this time.  There are no U32 registers supported in stream-out. This register is a buffer.
pub const STREAM_OUT3_BUFFER_U32: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(4416);
/// Data destination when sending 16-bit integer data. Each value uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT0_BUFFER_U16: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(4420);
/// Data destination when sending 16-bit integer data. Each value uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT1_BUFFER_U16: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(4421);
/// Data destination when sending 16-bit integer data. Each value uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT2_BUFFER_U16: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(4422);
/// Data destination when sending 16-bit integer data. Each value uses 2 bytes of the stream-out buffer. This register is a buffer.
pub const STREAM_OUT3_BUFFER_U16: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(4423);
/// Address to read stream data from when operating in C-R mode.
pub const STREAM_DATA_CR: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(4500);
/// Include one or more of these registers in STREAM_SCANLIST_ADDRESS#(0:127) to trigger stream-out updates.  When added to the scan list these do count against the max scan rate just like normal input addresses, but they do not return any data in the stream read.
pub const STREAM_OUT0: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(4800);
/// Include one or more of these registers in STREAM_SCANLIST_ADDRESS#(0:127) to trigger stream-out updates.  When added to the scan list these do count against the max scan rate just like normal input addresses, but they do not return any data in the stream read.
pub const STREAM_OUT1: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(4801);
/// Include one or more of these registers in STREAM_SCANLIST_ADDRESS#(0:127) to trigger stream-out updates.  When added to the scan list these do count against the max scan rate just like normal input addresses, but they do not return any data in the stream read.
pub const STREAM_OUT2: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(4802);
/// Include one or more of these registers in STREAM_SCANLIST_ADDRESS#(0:127) to trigger stream-out updates.  When added to the scan list these do count against the max scan rate just like normal input addresses, but they do not return any data in the stream read.
pub const STREAM_OUT3: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(4803);
/// Returns the index of the channel. The index is the position in the scan list that the channel occupies.
pub const STREAM_DEBUG_GET_SELF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4898);
/// If a channel produces 32-bits of data the upper 16 will be saved here.
pub const STREAM_DATA_CAPTURE_16: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(4899);
/// Write 1 to start stream. Write 0 to stop stream. Reading this register returns 1 when stream is enabled. When using a triggered stream the stream is considered enabled while waiting for the trigger.
pub const STREAM_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(4990);
/// The DIO line for Chip-Select.
pub const SPI_CS_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5000);
/// The DIO line for Clock.
pub const SPI_CLK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5001);
/// The DIO line for Master-In-Slave-Out.
pub const SPI_MISO_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5002);
/// The DIO line for Master-Out-Slave-In.
pub const SPI_MOSI_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5003);
/// The SPI mode controls the clock idle state and which edge clocks the data. Bit 1 is CPOL and Bit 0 is CPHA, so CPOL/CPHA for different decimal values: 0 = 0/0 = b00, 1 = 0/1 = b01, 2 = 1/0 = b10, 3 = 1/1 = b11. For CPOL and CPHA explanations, see Wikipedia article: https://en.wikipedia.org/wiki/Serial_Peripheral_Interface_Bus.
pub const SPI_MODE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5004);
/// This value controls the SPI clock frequency.  Pass 0-65535.  Default=0 corresponds to 65536 internally which results in ~800 kHz. 65500 = ~100 kHz, 65100 = ~10 kHz, 61100 = ~1 kHz, 21000 = ~100 Hz, and 1 = ~67 Hz. Avoid setting too low such that the entire transaction lasts longer than the 250 millisecond timeout of the internal watchdog timer.
pub const SPI_SPEED_THROTTLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5005);
/// Bit 0 is Auto-CS-Disable. When bit 0 is 0, CS is enabled. When bit 0 is 1, CS is disabled. Bit 1: 0 = Set DIO directions before starting the SPI operations, 1 = Do not set DIO directions. Bit 2: 0 = Transmit data MSB first, 1 = LSB first. Bits 4-7: This value sets the number of bits that will be transmitted during the last byte of the SPI operation. Default is 8, valid options are 1-8.
pub const SPI_OPTIONS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5006);
/// Write 1 to begin the configured SPI transaction.
pub const SPI_GO: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(5007);
/// The number of bytes to transfer. The maximum transfer size is 100 bytes.
pub const SPI_NUM_BYTES: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(5009);
/// Write data here. This register is a buffer.
pub const SPI_DATA_TX: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(5010);
/// Read data here. This register is a buffer. Underrun behavior - fill with zeros.
pub const SPI_DATA_RX: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(5050);
/// The number of the DIO line to be used as the I2C data line. Ex: Writing 0 will force FIO0 to become the I2C-SDA line.
pub const I2C_SDA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5100);
/// The number of the DIO line to be used as the I2C clock line. Ex: Writing 1 will force FIO1 to become the I2C-SCL line.
pub const I2C_SCL_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5101);
/// This value controls the I2C clock frequency.  Pass 0-65535. Default=0 corresponds to 65536 internally which results in ~450 kHz. 1 results in ~40 Hz, 65516 is ~100 kHz.
pub const I2C_SPEED_THROTTLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5102);
/// Advanced. Controls details of the I2C protocol to improve device compatibility. bit 0: 1 = Reset the I2C bus before attempting communication. bit 1: 0 = Restarts will use a stop and a start, 1 = Restarts will not use a stop. bit 2: 1 = disable clock stretching.
pub const I2C_OPTIONS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5103);
/// The 7-bit address of the slave device. Value is shifted left by firmware to allow room for the I2C R/W bit.
pub const I2C_SLAVE_ADDRESS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5104);
/// The number of data bytes to transmit. Zero is valid and will result in a read-only I2C operation.
pub const I2C_NUM_BYTES_TX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5108);
/// The number of data bytes to read. Zero is valid and will result in a write-only I2C operation.
pub const I2C_NUM_BYTES_RX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5109);
/// Writing to this register will instruct the LabJack to perform an I2C transaction.
pub const I2C_GO: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5110);
/// A binary encoded value (array of bits) used to observe ACKs from the slave device.
pub const I2C_ACKS: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(5114);
/// Data that will be written to the I2C bus. This register is a buffer.
pub const I2C_DATA_TX: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(5120);
/// Data that has been read from the I2C bus. This register is a buffer. Underrun behavior - fill with zeros.
pub const I2C_DATA_RX: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(5160);
/// The data-line DIO number.
pub const ONEWIRE_DQ_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5300);
/// The dynamic pullup control DIO number.
pub const ONEWIRE_DPU_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5301);
/// Controls advanced features. Value is a bitmask. bit 0: reserved, bit 1: reserved, bit 2: 1=DPU Enabled 0=DPU Disabled, bit 3: DPU Polarity 1=Active state is high, 0=Active state is low (Dynamic Pull-Up)
pub const ONEWIRE_OPTIONS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5302);
/// Set the ROM function to use. 0xF0=Search, 0xCC=Skip, 0x55=Match, 0x33=Read.
pub const ONEWIRE_FUNCTION: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5307);
/// Number of data bytes to be sent.
pub const ONEWIRE_NUM_BYTES_TX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5308);
/// Number of data bytes to be received.
pub const ONEWIRE_NUM_BYTES_RX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5309);
/// Instructs the device to perform the configured 1-wire transaction.
pub const ONEWIRE_GO: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(5310);
/// Upper 32-bits of the ROM to match.
pub const ONEWIRE_ROM_MATCH_H: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(5320);
/// Lower 32-bits of the ROM to match.
pub const ONEWIRE_ROM_MATCH_L: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(5322);
/// Upper 32-bits of the path to take during a search.
pub const ONEWIRE_PATH_H: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(5324);
/// Lower 32-bits of the path to take during a search.
pub const ONEWIRE_PATH_L: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(5326);
/// Upper 32-bits of the search result.
pub const ONEWIRE_SEARCH_RESULT_H: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(5328);
/// Lower 32-bites of the search result.
pub const ONEWIRE_SEARCH_RESULT_L: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(5330);
/// Upper 32-bits of the branches detected during a search.
pub const ONEWIRE_ROM_BRANCHS_FOUND_H: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(5332);
/// Lower 32-bits of the branches detected during a search.
pub const ONEWIRE_ROM_BRANCHS_FOUND_L: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(5334);
/// Data to be transmitted over the 1-wire bus. This register is a buffer.
pub const ONEWIRE_DATA_TX: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(5340);
/// Data received over the 1-wire bus. This register is a buffer. Underrun behavior - buffer is static, old data will fill the extra locations, firmware 1.0225 changes this to read zeros.
pub const ONEWIRE_DATA_RX: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(5370);
/// 1 = Turn on Asynch. Configures timing hardware, DIO lines and allocates the receiving buffer.
pub const ASYNCH_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5400);
/// The DIO line that will receive data. (RX)
pub const ASYNCH_RX_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5405);
/// The DIO line that will transmit data. (TX)
pub const ASYNCH_TX_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5410);
/// The number of data bits per frame. 0-8, 0=8.
pub const ASYNCH_NUM_DATA_BITS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5415);
/// The symbol rate that will be used for communication. 9600 is typical. Up to 38400 works, but heavily loads the T7's processor.
pub const ASYNCH_BAUD: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(5420);
/// Number of bytes to use for the receiving buffer. Max is 2048. 0 = 200
pub const ASYNCH_RX_BUFFER_SIZE_BYTES: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5430);
/// The number of data bytes that have been received.
pub const ASYNCH_NUM_BYTES_RX: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(5435);
/// The number of bytes to be transmitted after writing to GO. Max is 256.
pub const ASYNCH_NUM_BYTES_TX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5440);
/// Write a 1 to this register to initiate a transmission.
pub const ASYNCH_TX_GO: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(5450);
/// The number of stop bits. Values: 0 = zero stop bits, 1 = one stop bit, 2 = two stop bits.
pub const ASYNCH_NUM_STOP_BITS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5455);
/// Parity setting: 0=none, 1=odd, 2=even.
pub const ASYNCH_PARITY: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5460);
/// The number of parity errors that have been detected. Cleared when UART is enabled. Can also be cleared by writing 0.
pub const ASYNCH_NUM_PARITY_ERRORS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(5465);
/// Write data to be transmitted here. This register is a buffer.
pub const ASYNCH_DATA_TX: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(5490);
/// Read received data from here. This register is a buffer. Underrun behavior - fill with zeros.
pub const ASYNCH_DATA_RX: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(5495);
/// Writing 1 compiles and runs the Lua script that is loaded in RAM. Writing zero stops the script and begins cleaning up memory. Users may poll the register after writing a value of 0 to verify that the VM is unloaded, and garbage collection is complete. 0 = VM fully unloaded. 1 = Running/in-progress
pub const LUA_RUN: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6000);
/// Allocates RAM for the source code.
pub const LUA_SOURCE_SIZE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6012);
/// Write the source code here. Source will be saved to the RAM allocated by LUA_SOURCE_SIZE. This register is a buffer.
pub const LUA_SOURCE_WRITE: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(6014);
/// Write 1 to this register to enable debugging.
pub const LUA_DEBUG_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6020);
/// The number of data bytes in the debug buffer waiting to be read.
pub const LUA_DEBUG_NUM_BYTES: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(6022);
/// Read debug data from here. This register is a buffer.
pub const LUA_DEBUG_DATA: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(6024);
/// Write 1 to save the loaded source code to flash.
pub const LUA_SAVE_TO_FLASH: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6032);
/// Writing any value reads the script from Flash and loads it into RAM. The script can now be compiled and run.
pub const LUA_LOAD_SAVED: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(6034);
/// Address within the saved Lua script in Flash to begin reading from.
pub const LUA_SAVED_READ_POINTER: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6036);
/// Read script saved to flash from here.
pub const LUA_SAVED_READ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(6038);
/// Writing a 1 to this register will prevent truncation warnings from being displayed. This register will be cleared every time Lua is started.
pub const LUA_NO_WARN_TRUNCATION: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(6050);
/// If set to 1 the script saved in flash will be loaded and run when the LabJack boots up.
pub const LUA_RUN_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6100);
/// This is the value that will be loaded into LUA_DEBUG_ENABLE when the LabJack boots up.
pub const LUA_DEBUG_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6120);
/// This is the number of bytes that will be used for the lua debug buffer.
pub const LUA_DEBUG_NUM_BYTES_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(6122);
/// Function dependent on selected feature index.
pub const AIN0_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7000);
/// Function dependent on selected feature index.
pub const AIN1_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7002);
/// Function dependent on selected feature index.
pub const AIN2_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7004);
/// Function dependent on selected feature index.
pub const AIN3_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7006);
/// Function dependent on selected feature index.
pub const AIN4_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7008);
/// Function dependent on selected feature index.
pub const AIN5_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7010);
/// Function dependent on selected feature index.
pub const AIN6_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7012);
/// Function dependent on selected feature index.
pub const AIN7_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7014);
/// Function dependent on selected feature index.
pub const AIN8_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7016);
/// Function dependent on selected feature index.
pub const AIN9_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7018);
/// Function dependent on selected feature index.
pub const AIN10_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7020);
/// Function dependent on selected feature index.
pub const AIN11_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7022);
/// Function dependent on selected feature index.
pub const AIN12_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7024);
/// Function dependent on selected feature index.
pub const AIN13_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7026);
/// Function dependent on selected feature index.
pub const AIN14_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7028);
/// Function dependent on selected feature index.
pub const AIN15_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7030);
/// Function dependent on selected feature index.
pub const AIN16_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7032);
/// Function dependent on selected feature index.
pub const AIN17_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7034);
/// Function dependent on selected feature index.
pub const AIN18_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7036);
/// Function dependent on selected feature index.
pub const AIN19_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7038);
/// Function dependent on selected feature index.
pub const AIN20_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7040);
/// Function dependent on selected feature index.
pub const AIN21_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7042);
/// Function dependent on selected feature index.
pub const AIN22_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7044);
/// Function dependent on selected feature index.
pub const AIN23_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7046);
/// Function dependent on selected feature index.
pub const AIN24_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7048);
/// Function dependent on selected feature index.
pub const AIN25_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7050);
/// Function dependent on selected feature index.
pub const AIN26_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7052);
/// Function dependent on selected feature index.
pub const AIN27_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7054);
/// Function dependent on selected feature index.
pub const AIN28_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7056);
/// Function dependent on selected feature index.
pub const AIN29_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7058);
/// Function dependent on selected feature index.
pub const AIN30_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7060);
/// Function dependent on selected feature index.
pub const AIN31_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7062);
/// Function dependent on selected feature index.
pub const AIN32_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7064);
/// Function dependent on selected feature index.
pub const AIN33_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7066);
/// Function dependent on selected feature index.
pub const AIN34_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7068);
/// Function dependent on selected feature index.
pub const AIN35_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7070);
/// Function dependent on selected feature index.
pub const AIN36_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7072);
/// Function dependent on selected feature index.
pub const AIN37_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7074);
/// Function dependent on selected feature index.
pub const AIN38_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7076);
/// Function dependent on selected feature index.
pub const AIN39_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7078);
/// Function dependent on selected feature index.
pub const AIN40_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7080);
/// Function dependent on selected feature index.
pub const AIN41_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7082);
/// Function dependent on selected feature index.
pub const AIN42_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7084);
/// Function dependent on selected feature index.
pub const AIN43_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7086);
/// Function dependent on selected feature index.
pub const AIN44_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7088);
/// Function dependent on selected feature index.
pub const AIN45_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7090);
/// Function dependent on selected feature index.
pub const AIN46_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7092);
/// Function dependent on selected feature index.
pub const AIN47_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7094);
/// Function dependent on selected feature index.
pub const AIN48_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7096);
/// Function dependent on selected feature index.
pub const AIN49_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7098);
/// Function dependent on selected feature index.
pub const AIN50_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7100);
/// Function dependent on selected feature index.
pub const AIN51_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7102);
/// Function dependent on selected feature index.
pub const AIN52_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7104);
/// Function dependent on selected feature index.
pub const AIN53_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7106);
/// Function dependent on selected feature index.
pub const AIN54_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7108);
/// Function dependent on selected feature index.
pub const AIN55_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7110);
/// Function dependent on selected feature index.
pub const AIN56_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7112);
/// Function dependent on selected feature index.
pub const AIN57_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7114);
/// Function dependent on selected feature index.
pub const AIN58_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7116);
/// Function dependent on selected feature index.
pub const AIN59_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7118);
/// Function dependent on selected feature index.
pub const AIN60_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7120);
/// Function dependent on selected feature index.
pub const AIN61_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7122);
/// Function dependent on selected feature index.
pub const AIN62_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7124);
/// Function dependent on selected feature index.
pub const AIN63_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7126);
/// Function dependent on selected feature index.
pub const AIN64_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7128);
/// Function dependent on selected feature index.
pub const AIN65_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7130);
/// Function dependent on selected feature index.
pub const AIN66_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7132);
/// Function dependent on selected feature index.
pub const AIN67_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7134);
/// Function dependent on selected feature index.
pub const AIN68_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7136);
/// Function dependent on selected feature index.
pub const AIN69_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7138);
/// Function dependent on selected feature index.
pub const AIN70_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7140);
/// Function dependent on selected feature index.
pub const AIN71_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7142);
/// Function dependent on selected feature index.
pub const AIN72_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7144);
/// Function dependent on selected feature index.
pub const AIN73_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7146);
/// Function dependent on selected feature index.
pub const AIN74_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7148);
/// Function dependent on selected feature index.
pub const AIN75_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7150);
/// Function dependent on selected feature index.
pub const AIN76_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7152);
/// Function dependent on selected feature index.
pub const AIN77_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7154);
/// Function dependent on selected feature index.
pub const AIN78_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7156);
/// Function dependent on selected feature index.
pub const AIN79_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7158);
/// Function dependent on selected feature index.
pub const AIN80_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7160);
/// Function dependent on selected feature index.
pub const AIN81_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7162);
/// Function dependent on selected feature index.
pub const AIN82_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7164);
/// Function dependent on selected feature index.
pub const AIN83_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7166);
/// Function dependent on selected feature index.
pub const AIN84_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7168);
/// Function dependent on selected feature index.
pub const AIN85_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7170);
/// Function dependent on selected feature index.
pub const AIN86_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7172);
/// Function dependent on selected feature index.
pub const AIN87_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7174);
/// Function dependent on selected feature index.
pub const AIN88_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7176);
/// Function dependent on selected feature index.
pub const AIN89_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7178);
/// Function dependent on selected feature index.
pub const AIN90_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7180);
/// Function dependent on selected feature index.
pub const AIN91_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7182);
/// Function dependent on selected feature index.
pub const AIN92_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7184);
/// Function dependent on selected feature index.
pub const AIN93_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7186);
/// Function dependent on selected feature index.
pub const AIN94_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7188);
/// Function dependent on selected feature index.
pub const AIN95_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7190);
/// Function dependent on selected feature index.
pub const AIN96_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7192);
/// Function dependent on selected feature index.
pub const AIN97_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7194);
/// Function dependent on selected feature index.
pub const AIN98_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7196);
/// Function dependent on selected feature index.
pub const AIN99_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7198);
/// Function dependent on selected feature index.
pub const AIN100_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7200);
/// Function dependent on selected feature index.
pub const AIN101_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7202);
/// Function dependent on selected feature index.
pub const AIN102_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7204);
/// Function dependent on selected feature index.
pub const AIN103_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7206);
/// Function dependent on selected feature index.
pub const AIN104_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7208);
/// Function dependent on selected feature index.
pub const AIN105_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7210);
/// Function dependent on selected feature index.
pub const AIN106_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7212);
/// Function dependent on selected feature index.
pub const AIN107_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7214);
/// Function dependent on selected feature index.
pub const AIN108_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7216);
/// Function dependent on selected feature index.
pub const AIN109_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7218);
/// Function dependent on selected feature index.
pub const AIN110_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7220);
/// Function dependent on selected feature index.
pub const AIN111_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7222);
/// Function dependent on selected feature index.
pub const AIN112_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7224);
/// Function dependent on selected feature index.
pub const AIN113_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7226);
/// Function dependent on selected feature index.
pub const AIN114_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7228);
/// Function dependent on selected feature index.
pub const AIN115_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7230);
/// Function dependent on selected feature index.
pub const AIN116_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7232);
/// Function dependent on selected feature index.
pub const AIN117_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7234);
/// Function dependent on selected feature index.
pub const AIN118_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7236);
/// Function dependent on selected feature index.
pub const AIN119_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7238);
/// Function dependent on selected feature index.
pub const AIN120_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7240);
/// Function dependent on selected feature index.
pub const AIN121_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7242);
/// Function dependent on selected feature index.
pub const AIN122_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7244);
/// Function dependent on selected feature index.
pub const AIN123_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7246);
/// Function dependent on selected feature index.
pub const AIN124_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7248);
/// Function dependent on selected feature index.
pub const AIN125_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7250);
/// Function dependent on selected feature index.
pub const AIN126_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7252);
/// Function dependent on selected feature index.
pub const AIN127_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7254);
/// Function dependent on selected feature index.
pub const AIN128_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7256);
/// Function dependent on selected feature index.
pub const AIN129_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7258);
/// Function dependent on selected feature index.
pub const AIN130_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7260);
/// Function dependent on selected feature index.
pub const AIN131_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7262);
/// Function dependent on selected feature index.
pub const AIN132_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7264);
/// Function dependent on selected feature index.
pub const AIN133_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7266);
/// Function dependent on selected feature index.
pub const AIN134_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7268);
/// Function dependent on selected feature index.
pub const AIN135_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7270);
/// Function dependent on selected feature index.
pub const AIN136_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7272);
/// Function dependent on selected feature index.
pub const AIN137_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7274);
/// Function dependent on selected feature index.
pub const AIN138_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7276);
/// Function dependent on selected feature index.
pub const AIN139_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7278);
/// Function dependent on selected feature index.
pub const AIN140_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7280);
/// Function dependent on selected feature index.
pub const AIN141_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7282);
/// Function dependent on selected feature index.
pub const AIN142_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7284);
/// Function dependent on selected feature index.
pub const AIN143_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7286);
/// Function dependent on selected feature index.
pub const AIN144_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7288);
/// Function dependent on selected feature index.
pub const AIN145_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7290);
/// Function dependent on selected feature index.
pub const AIN146_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7292);
/// Function dependent on selected feature index.
pub const AIN147_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7294);
/// Function dependent on selected feature index.
pub const AIN148_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7296);
/// Function dependent on selected feature index.
pub const AIN149_EF_READ_A: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7298);
/// Function dependent on selected feature index.
pub const AIN0_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7300);
/// Function dependent on selected feature index.
pub const AIN1_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7302);
/// Function dependent on selected feature index.
pub const AIN2_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7304);
/// Function dependent on selected feature index.
pub const AIN3_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7306);
/// Function dependent on selected feature index.
pub const AIN4_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7308);
/// Function dependent on selected feature index.
pub const AIN5_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7310);
/// Function dependent on selected feature index.
pub const AIN6_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7312);
/// Function dependent on selected feature index.
pub const AIN7_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7314);
/// Function dependent on selected feature index.
pub const AIN8_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7316);
/// Function dependent on selected feature index.
pub const AIN9_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7318);
/// Function dependent on selected feature index.
pub const AIN10_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7320);
/// Function dependent on selected feature index.
pub const AIN11_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7322);
/// Function dependent on selected feature index.
pub const AIN12_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7324);
/// Function dependent on selected feature index.
pub const AIN13_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7326);
/// Function dependent on selected feature index.
pub const AIN14_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7328);
/// Function dependent on selected feature index.
pub const AIN15_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7330);
/// Function dependent on selected feature index.
pub const AIN16_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7332);
/// Function dependent on selected feature index.
pub const AIN17_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7334);
/// Function dependent on selected feature index.
pub const AIN18_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7336);
/// Function dependent on selected feature index.
pub const AIN19_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7338);
/// Function dependent on selected feature index.
pub const AIN20_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7340);
/// Function dependent on selected feature index.
pub const AIN21_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7342);
/// Function dependent on selected feature index.
pub const AIN22_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7344);
/// Function dependent on selected feature index.
pub const AIN23_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7346);
/// Function dependent on selected feature index.
pub const AIN24_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7348);
/// Function dependent on selected feature index.
pub const AIN25_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7350);
/// Function dependent on selected feature index.
pub const AIN26_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7352);
/// Function dependent on selected feature index.
pub const AIN27_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7354);
/// Function dependent on selected feature index.
pub const AIN28_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7356);
/// Function dependent on selected feature index.
pub const AIN29_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7358);
/// Function dependent on selected feature index.
pub const AIN30_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7360);
/// Function dependent on selected feature index.
pub const AIN31_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7362);
/// Function dependent on selected feature index.
pub const AIN32_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7364);
/// Function dependent on selected feature index.
pub const AIN33_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7366);
/// Function dependent on selected feature index.
pub const AIN34_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7368);
/// Function dependent on selected feature index.
pub const AIN35_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7370);
/// Function dependent on selected feature index.
pub const AIN36_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7372);
/// Function dependent on selected feature index.
pub const AIN37_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7374);
/// Function dependent on selected feature index.
pub const AIN38_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7376);
/// Function dependent on selected feature index.
pub const AIN39_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7378);
/// Function dependent on selected feature index.
pub const AIN40_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7380);
/// Function dependent on selected feature index.
pub const AIN41_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7382);
/// Function dependent on selected feature index.
pub const AIN42_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7384);
/// Function dependent on selected feature index.
pub const AIN43_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7386);
/// Function dependent on selected feature index.
pub const AIN44_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7388);
/// Function dependent on selected feature index.
pub const AIN45_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7390);
/// Function dependent on selected feature index.
pub const AIN46_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7392);
/// Function dependent on selected feature index.
pub const AIN47_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7394);
/// Function dependent on selected feature index.
pub const AIN48_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7396);
/// Function dependent on selected feature index.
pub const AIN49_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7398);
/// Function dependent on selected feature index.
pub const AIN50_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7400);
/// Function dependent on selected feature index.
pub const AIN51_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7402);
/// Function dependent on selected feature index.
pub const AIN52_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7404);
/// Function dependent on selected feature index.
pub const AIN53_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7406);
/// Function dependent on selected feature index.
pub const AIN54_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7408);
/// Function dependent on selected feature index.
pub const AIN55_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7410);
/// Function dependent on selected feature index.
pub const AIN56_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7412);
/// Function dependent on selected feature index.
pub const AIN57_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7414);
/// Function dependent on selected feature index.
pub const AIN58_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7416);
/// Function dependent on selected feature index.
pub const AIN59_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7418);
/// Function dependent on selected feature index.
pub const AIN60_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7420);
/// Function dependent on selected feature index.
pub const AIN61_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7422);
/// Function dependent on selected feature index.
pub const AIN62_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7424);
/// Function dependent on selected feature index.
pub const AIN63_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7426);
/// Function dependent on selected feature index.
pub const AIN64_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7428);
/// Function dependent on selected feature index.
pub const AIN65_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7430);
/// Function dependent on selected feature index.
pub const AIN66_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7432);
/// Function dependent on selected feature index.
pub const AIN67_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7434);
/// Function dependent on selected feature index.
pub const AIN68_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7436);
/// Function dependent on selected feature index.
pub const AIN69_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7438);
/// Function dependent on selected feature index.
pub const AIN70_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7440);
/// Function dependent on selected feature index.
pub const AIN71_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7442);
/// Function dependent on selected feature index.
pub const AIN72_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7444);
/// Function dependent on selected feature index.
pub const AIN73_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7446);
/// Function dependent on selected feature index.
pub const AIN74_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7448);
/// Function dependent on selected feature index.
pub const AIN75_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7450);
/// Function dependent on selected feature index.
pub const AIN76_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7452);
/// Function dependent on selected feature index.
pub const AIN77_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7454);
/// Function dependent on selected feature index.
pub const AIN78_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7456);
/// Function dependent on selected feature index.
pub const AIN79_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7458);
/// Function dependent on selected feature index.
pub const AIN80_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7460);
/// Function dependent on selected feature index.
pub const AIN81_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7462);
/// Function dependent on selected feature index.
pub const AIN82_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7464);
/// Function dependent on selected feature index.
pub const AIN83_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7466);
/// Function dependent on selected feature index.
pub const AIN84_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7468);
/// Function dependent on selected feature index.
pub const AIN85_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7470);
/// Function dependent on selected feature index.
pub const AIN86_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7472);
/// Function dependent on selected feature index.
pub const AIN87_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7474);
/// Function dependent on selected feature index.
pub const AIN88_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7476);
/// Function dependent on selected feature index.
pub const AIN89_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7478);
/// Function dependent on selected feature index.
pub const AIN90_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7480);
/// Function dependent on selected feature index.
pub const AIN91_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7482);
/// Function dependent on selected feature index.
pub const AIN92_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7484);
/// Function dependent on selected feature index.
pub const AIN93_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7486);
/// Function dependent on selected feature index.
pub const AIN94_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7488);
/// Function dependent on selected feature index.
pub const AIN95_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7490);
/// Function dependent on selected feature index.
pub const AIN96_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7492);
/// Function dependent on selected feature index.
pub const AIN97_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7494);
/// Function dependent on selected feature index.
pub const AIN98_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7496);
/// Function dependent on selected feature index.
pub const AIN99_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7498);
/// Function dependent on selected feature index.
pub const AIN100_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7500);
/// Function dependent on selected feature index.
pub const AIN101_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7502);
/// Function dependent on selected feature index.
pub const AIN102_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7504);
/// Function dependent on selected feature index.
pub const AIN103_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7506);
/// Function dependent on selected feature index.
pub const AIN104_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7508);
/// Function dependent on selected feature index.
pub const AIN105_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7510);
/// Function dependent on selected feature index.
pub const AIN106_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7512);
/// Function dependent on selected feature index.
pub const AIN107_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7514);
/// Function dependent on selected feature index.
pub const AIN108_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7516);
/// Function dependent on selected feature index.
pub const AIN109_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7518);
/// Function dependent on selected feature index.
pub const AIN110_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7520);
/// Function dependent on selected feature index.
pub const AIN111_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7522);
/// Function dependent on selected feature index.
pub const AIN112_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7524);
/// Function dependent on selected feature index.
pub const AIN113_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7526);
/// Function dependent on selected feature index.
pub const AIN114_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7528);
/// Function dependent on selected feature index.
pub const AIN115_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7530);
/// Function dependent on selected feature index.
pub const AIN116_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7532);
/// Function dependent on selected feature index.
pub const AIN117_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7534);
/// Function dependent on selected feature index.
pub const AIN118_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7536);
/// Function dependent on selected feature index.
pub const AIN119_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7538);
/// Function dependent on selected feature index.
pub const AIN120_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7540);
/// Function dependent on selected feature index.
pub const AIN121_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7542);
/// Function dependent on selected feature index.
pub const AIN122_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7544);
/// Function dependent on selected feature index.
pub const AIN123_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7546);
/// Function dependent on selected feature index.
pub const AIN124_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7548);
/// Function dependent on selected feature index.
pub const AIN125_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7550);
/// Function dependent on selected feature index.
pub const AIN126_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7552);
/// Function dependent on selected feature index.
pub const AIN127_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7554);
/// Function dependent on selected feature index.
pub const AIN128_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7556);
/// Function dependent on selected feature index.
pub const AIN129_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7558);
/// Function dependent on selected feature index.
pub const AIN130_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7560);
/// Function dependent on selected feature index.
pub const AIN131_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7562);
/// Function dependent on selected feature index.
pub const AIN132_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7564);
/// Function dependent on selected feature index.
pub const AIN133_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7566);
/// Function dependent on selected feature index.
pub const AIN134_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7568);
/// Function dependent on selected feature index.
pub const AIN135_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7570);
/// Function dependent on selected feature index.
pub const AIN136_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7572);
/// Function dependent on selected feature index.
pub const AIN137_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7574);
/// Function dependent on selected feature index.
pub const AIN138_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7576);
/// Function dependent on selected feature index.
pub const AIN139_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7578);
/// Function dependent on selected feature index.
pub const AIN140_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7580);
/// Function dependent on selected feature index.
pub const AIN141_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7582);
/// Function dependent on selected feature index.
pub const AIN142_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7584);
/// Function dependent on selected feature index.
pub const AIN143_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7586);
/// Function dependent on selected feature index.
pub const AIN144_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7588);
/// Function dependent on selected feature index.
pub const AIN145_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7590);
/// Function dependent on selected feature index.
pub const AIN146_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7592);
/// Function dependent on selected feature index.
pub const AIN147_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7594);
/// Function dependent on selected feature index.
pub const AIN148_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7596);
/// Function dependent on selected feature index.
pub const AIN149_EF_READ_B: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7598);
/// Function dependent on selected feature index.
pub const AIN0_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7600);
/// Function dependent on selected feature index.
pub const AIN1_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7602);
/// Function dependent on selected feature index.
pub const AIN2_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7604);
/// Function dependent on selected feature index.
pub const AIN3_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7606);
/// Function dependent on selected feature index.
pub const AIN4_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7608);
/// Function dependent on selected feature index.
pub const AIN5_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7610);
/// Function dependent on selected feature index.
pub const AIN6_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7612);
/// Function dependent on selected feature index.
pub const AIN7_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7614);
/// Function dependent on selected feature index.
pub const AIN8_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7616);
/// Function dependent on selected feature index.
pub const AIN9_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7618);
/// Function dependent on selected feature index.
pub const AIN10_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7620);
/// Function dependent on selected feature index.
pub const AIN11_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7622);
/// Function dependent on selected feature index.
pub const AIN12_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7624);
/// Function dependent on selected feature index.
pub const AIN13_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7626);
/// Function dependent on selected feature index.
pub const AIN14_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7628);
/// Function dependent on selected feature index.
pub const AIN15_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7630);
/// Function dependent on selected feature index.
pub const AIN16_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7632);
/// Function dependent on selected feature index.
pub const AIN17_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7634);
/// Function dependent on selected feature index.
pub const AIN18_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7636);
/// Function dependent on selected feature index.
pub const AIN19_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7638);
/// Function dependent on selected feature index.
pub const AIN20_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7640);
/// Function dependent on selected feature index.
pub const AIN21_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7642);
/// Function dependent on selected feature index.
pub const AIN22_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7644);
/// Function dependent on selected feature index.
pub const AIN23_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7646);
/// Function dependent on selected feature index.
pub const AIN24_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7648);
/// Function dependent on selected feature index.
pub const AIN25_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7650);
/// Function dependent on selected feature index.
pub const AIN26_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7652);
/// Function dependent on selected feature index.
pub const AIN27_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7654);
/// Function dependent on selected feature index.
pub const AIN28_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7656);
/// Function dependent on selected feature index.
pub const AIN29_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7658);
/// Function dependent on selected feature index.
pub const AIN30_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7660);
/// Function dependent on selected feature index.
pub const AIN31_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7662);
/// Function dependent on selected feature index.
pub const AIN32_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7664);
/// Function dependent on selected feature index.
pub const AIN33_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7666);
/// Function dependent on selected feature index.
pub const AIN34_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7668);
/// Function dependent on selected feature index.
pub const AIN35_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7670);
/// Function dependent on selected feature index.
pub const AIN36_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7672);
/// Function dependent on selected feature index.
pub const AIN37_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7674);
/// Function dependent on selected feature index.
pub const AIN38_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7676);
/// Function dependent on selected feature index.
pub const AIN39_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7678);
/// Function dependent on selected feature index.
pub const AIN40_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7680);
/// Function dependent on selected feature index.
pub const AIN41_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7682);
/// Function dependent on selected feature index.
pub const AIN42_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7684);
/// Function dependent on selected feature index.
pub const AIN43_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7686);
/// Function dependent on selected feature index.
pub const AIN44_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7688);
/// Function dependent on selected feature index.
pub const AIN45_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7690);
/// Function dependent on selected feature index.
pub const AIN46_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7692);
/// Function dependent on selected feature index.
pub const AIN47_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7694);
/// Function dependent on selected feature index.
pub const AIN48_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7696);
/// Function dependent on selected feature index.
pub const AIN49_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7698);
/// Function dependent on selected feature index.
pub const AIN50_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7700);
/// Function dependent on selected feature index.
pub const AIN51_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7702);
/// Function dependent on selected feature index.
pub const AIN52_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7704);
/// Function dependent on selected feature index.
pub const AIN53_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7706);
/// Function dependent on selected feature index.
pub const AIN54_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7708);
/// Function dependent on selected feature index.
pub const AIN55_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7710);
/// Function dependent on selected feature index.
pub const AIN56_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7712);
/// Function dependent on selected feature index.
pub const AIN57_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7714);
/// Function dependent on selected feature index.
pub const AIN58_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7716);
/// Function dependent on selected feature index.
pub const AIN59_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7718);
/// Function dependent on selected feature index.
pub const AIN60_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7720);
/// Function dependent on selected feature index.
pub const AIN61_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7722);
/// Function dependent on selected feature index.
pub const AIN62_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7724);
/// Function dependent on selected feature index.
pub const AIN63_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7726);
/// Function dependent on selected feature index.
pub const AIN64_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7728);
/// Function dependent on selected feature index.
pub const AIN65_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7730);
/// Function dependent on selected feature index.
pub const AIN66_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7732);
/// Function dependent on selected feature index.
pub const AIN67_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7734);
/// Function dependent on selected feature index.
pub const AIN68_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7736);
/// Function dependent on selected feature index.
pub const AIN69_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7738);
/// Function dependent on selected feature index.
pub const AIN70_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7740);
/// Function dependent on selected feature index.
pub const AIN71_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7742);
/// Function dependent on selected feature index.
pub const AIN72_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7744);
/// Function dependent on selected feature index.
pub const AIN73_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7746);
/// Function dependent on selected feature index.
pub const AIN74_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7748);
/// Function dependent on selected feature index.
pub const AIN75_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7750);
/// Function dependent on selected feature index.
pub const AIN76_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7752);
/// Function dependent on selected feature index.
pub const AIN77_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7754);
/// Function dependent on selected feature index.
pub const AIN78_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7756);
/// Function dependent on selected feature index.
pub const AIN79_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7758);
/// Function dependent on selected feature index.
pub const AIN80_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7760);
/// Function dependent on selected feature index.
pub const AIN81_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7762);
/// Function dependent on selected feature index.
pub const AIN82_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7764);
/// Function dependent on selected feature index.
pub const AIN83_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7766);
/// Function dependent on selected feature index.
pub const AIN84_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7768);
/// Function dependent on selected feature index.
pub const AIN85_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7770);
/// Function dependent on selected feature index.
pub const AIN86_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7772);
/// Function dependent on selected feature index.
pub const AIN87_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7774);
/// Function dependent on selected feature index.
pub const AIN88_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7776);
/// Function dependent on selected feature index.
pub const AIN89_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7778);
/// Function dependent on selected feature index.
pub const AIN90_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7780);
/// Function dependent on selected feature index.
pub const AIN91_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7782);
/// Function dependent on selected feature index.
pub const AIN92_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7784);
/// Function dependent on selected feature index.
pub const AIN93_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7786);
/// Function dependent on selected feature index.
pub const AIN94_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7788);
/// Function dependent on selected feature index.
pub const AIN95_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7790);
/// Function dependent on selected feature index.
pub const AIN96_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7792);
/// Function dependent on selected feature index.
pub const AIN97_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7794);
/// Function dependent on selected feature index.
pub const AIN98_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7796);
/// Function dependent on selected feature index.
pub const AIN99_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7798);
/// Function dependent on selected feature index.
pub const AIN100_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7800);
/// Function dependent on selected feature index.
pub const AIN101_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7802);
/// Function dependent on selected feature index.
pub const AIN102_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7804);
/// Function dependent on selected feature index.
pub const AIN103_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7806);
/// Function dependent on selected feature index.
pub const AIN104_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7808);
/// Function dependent on selected feature index.
pub const AIN105_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7810);
/// Function dependent on selected feature index.
pub const AIN106_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7812);
/// Function dependent on selected feature index.
pub const AIN107_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7814);
/// Function dependent on selected feature index.
pub const AIN108_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7816);
/// Function dependent on selected feature index.
pub const AIN109_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7818);
/// Function dependent on selected feature index.
pub const AIN110_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7820);
/// Function dependent on selected feature index.
pub const AIN111_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7822);
/// Function dependent on selected feature index.
pub const AIN112_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7824);
/// Function dependent on selected feature index.
pub const AIN113_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7826);
/// Function dependent on selected feature index.
pub const AIN114_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7828);
/// Function dependent on selected feature index.
pub const AIN115_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7830);
/// Function dependent on selected feature index.
pub const AIN116_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7832);
/// Function dependent on selected feature index.
pub const AIN117_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7834);
/// Function dependent on selected feature index.
pub const AIN118_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7836);
/// Function dependent on selected feature index.
pub const AIN119_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7838);
/// Function dependent on selected feature index.
pub const AIN120_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7840);
/// Function dependent on selected feature index.
pub const AIN121_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7842);
/// Function dependent on selected feature index.
pub const AIN122_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7844);
/// Function dependent on selected feature index.
pub const AIN123_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7846);
/// Function dependent on selected feature index.
pub const AIN124_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7848);
/// Function dependent on selected feature index.
pub const AIN125_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7850);
/// Function dependent on selected feature index.
pub const AIN126_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7852);
/// Function dependent on selected feature index.
pub const AIN127_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7854);
/// Function dependent on selected feature index.
pub const AIN128_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7856);
/// Function dependent on selected feature index.
pub const AIN129_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7858);
/// Function dependent on selected feature index.
pub const AIN130_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7860);
/// Function dependent on selected feature index.
pub const AIN131_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7862);
/// Function dependent on selected feature index.
pub const AIN132_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7864);
/// Function dependent on selected feature index.
pub const AIN133_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7866);
/// Function dependent on selected feature index.
pub const AIN134_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7868);
/// Function dependent on selected feature index.
pub const AIN135_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7870);
/// Function dependent on selected feature index.
pub const AIN136_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7872);
/// Function dependent on selected feature index.
pub const AIN137_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7874);
/// Function dependent on selected feature index.
pub const AIN138_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7876);
/// Function dependent on selected feature index.
pub const AIN139_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7878);
/// Function dependent on selected feature index.
pub const AIN140_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7880);
/// Function dependent on selected feature index.
pub const AIN141_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7882);
/// Function dependent on selected feature index.
pub const AIN142_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7884);
/// Function dependent on selected feature index.
pub const AIN143_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7886);
/// Function dependent on selected feature index.
pub const AIN144_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7888);
/// Function dependent on selected feature index.
pub const AIN145_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7890);
/// Function dependent on selected feature index.
pub const AIN146_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7892);
/// Function dependent on selected feature index.
pub const AIN147_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7894);
/// Function dependent on selected feature index.
pub const AIN148_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7896);
/// Function dependent on selected feature index.
pub const AIN149_EF_READ_C: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(7898);
/// Function dependent on selected feature index.
pub const AIN0_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7900);
/// Function dependent on selected feature index.
pub const AIN1_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7902);
/// Function dependent on selected feature index.
pub const AIN2_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7904);
/// Function dependent on selected feature index.
pub const AIN3_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7906);
/// Function dependent on selected feature index.
pub const AIN4_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7908);
/// Function dependent on selected feature index.
pub const AIN5_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7910);
/// Function dependent on selected feature index.
pub const AIN6_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7912);
/// Function dependent on selected feature index.
pub const AIN7_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7914);
/// Function dependent on selected feature index.
pub const AIN8_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7916);
/// Function dependent on selected feature index.
pub const AIN9_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7918);
/// Function dependent on selected feature index.
pub const AIN10_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7920);
/// Function dependent on selected feature index.
pub const AIN11_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7922);
/// Function dependent on selected feature index.
pub const AIN12_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7924);
/// Function dependent on selected feature index.
pub const AIN13_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7926);
/// Function dependent on selected feature index.
pub const AIN14_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7928);
/// Function dependent on selected feature index.
pub const AIN15_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7930);
/// Function dependent on selected feature index.
pub const AIN16_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7932);
/// Function dependent on selected feature index.
pub const AIN17_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7934);
/// Function dependent on selected feature index.
pub const AIN18_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7936);
/// Function dependent on selected feature index.
pub const AIN19_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7938);
/// Function dependent on selected feature index.
pub const AIN20_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7940);
/// Function dependent on selected feature index.
pub const AIN21_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7942);
/// Function dependent on selected feature index.
pub const AIN22_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7944);
/// Function dependent on selected feature index.
pub const AIN23_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7946);
/// Function dependent on selected feature index.
pub const AIN24_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7948);
/// Function dependent on selected feature index.
pub const AIN25_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7950);
/// Function dependent on selected feature index.
pub const AIN26_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7952);
/// Function dependent on selected feature index.
pub const AIN27_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7954);
/// Function dependent on selected feature index.
pub const AIN28_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7956);
/// Function dependent on selected feature index.
pub const AIN29_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7958);
/// Function dependent on selected feature index.
pub const AIN30_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7960);
/// Function dependent on selected feature index.
pub const AIN31_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7962);
/// Function dependent on selected feature index.
pub const AIN32_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7964);
/// Function dependent on selected feature index.
pub const AIN33_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7966);
/// Function dependent on selected feature index.
pub const AIN34_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7968);
/// Function dependent on selected feature index.
pub const AIN35_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7970);
/// Function dependent on selected feature index.
pub const AIN36_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7972);
/// Function dependent on selected feature index.
pub const AIN37_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7974);
/// Function dependent on selected feature index.
pub const AIN38_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7976);
/// Function dependent on selected feature index.
pub const AIN39_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7978);
/// Function dependent on selected feature index.
pub const AIN40_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7980);
/// Function dependent on selected feature index.
pub const AIN41_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7982);
/// Function dependent on selected feature index.
pub const AIN42_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7984);
/// Function dependent on selected feature index.
pub const AIN43_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7986);
/// Function dependent on selected feature index.
pub const AIN44_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7988);
/// Function dependent on selected feature index.
pub const AIN45_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7990);
/// Function dependent on selected feature index.
pub const AIN46_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7992);
/// Function dependent on selected feature index.
pub const AIN47_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7994);
/// Function dependent on selected feature index.
pub const AIN48_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7996);
/// Function dependent on selected feature index.
pub const AIN49_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(7998);
/// Function dependent on selected feature index.
pub const AIN50_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8000);
/// Function dependent on selected feature index.
pub const AIN51_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8002);
/// Function dependent on selected feature index.
pub const AIN52_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8004);
/// Function dependent on selected feature index.
pub const AIN53_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8006);
/// Function dependent on selected feature index.
pub const AIN54_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8008);
/// Function dependent on selected feature index.
pub const AIN55_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8010);
/// Function dependent on selected feature index.
pub const AIN56_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8012);
/// Function dependent on selected feature index.
pub const AIN57_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8014);
/// Function dependent on selected feature index.
pub const AIN58_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8016);
/// Function dependent on selected feature index.
pub const AIN59_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8018);
/// Function dependent on selected feature index.
pub const AIN60_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8020);
/// Function dependent on selected feature index.
pub const AIN61_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8022);
/// Function dependent on selected feature index.
pub const AIN62_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8024);
/// Function dependent on selected feature index.
pub const AIN63_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8026);
/// Function dependent on selected feature index.
pub const AIN64_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8028);
/// Function dependent on selected feature index.
pub const AIN65_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8030);
/// Function dependent on selected feature index.
pub const AIN66_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8032);
/// Function dependent on selected feature index.
pub const AIN67_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8034);
/// Function dependent on selected feature index.
pub const AIN68_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8036);
/// Function dependent on selected feature index.
pub const AIN69_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8038);
/// Function dependent on selected feature index.
pub const AIN70_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8040);
/// Function dependent on selected feature index.
pub const AIN71_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8042);
/// Function dependent on selected feature index.
pub const AIN72_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8044);
/// Function dependent on selected feature index.
pub const AIN73_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8046);
/// Function dependent on selected feature index.
pub const AIN74_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8048);
/// Function dependent on selected feature index.
pub const AIN75_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8050);
/// Function dependent on selected feature index.
pub const AIN76_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8052);
/// Function dependent on selected feature index.
pub const AIN77_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8054);
/// Function dependent on selected feature index.
pub const AIN78_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8056);
/// Function dependent on selected feature index.
pub const AIN79_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8058);
/// Function dependent on selected feature index.
pub const AIN80_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8060);
/// Function dependent on selected feature index.
pub const AIN81_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8062);
/// Function dependent on selected feature index.
pub const AIN82_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8064);
/// Function dependent on selected feature index.
pub const AIN83_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8066);
/// Function dependent on selected feature index.
pub const AIN84_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8068);
/// Function dependent on selected feature index.
pub const AIN85_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8070);
/// Function dependent on selected feature index.
pub const AIN86_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8072);
/// Function dependent on selected feature index.
pub const AIN87_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8074);
/// Function dependent on selected feature index.
pub const AIN88_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8076);
/// Function dependent on selected feature index.
pub const AIN89_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8078);
/// Function dependent on selected feature index.
pub const AIN90_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8080);
/// Function dependent on selected feature index.
pub const AIN91_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8082);
/// Function dependent on selected feature index.
pub const AIN92_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8084);
/// Function dependent on selected feature index.
pub const AIN93_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8086);
/// Function dependent on selected feature index.
pub const AIN94_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8088);
/// Function dependent on selected feature index.
pub const AIN95_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8090);
/// Function dependent on selected feature index.
pub const AIN96_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8092);
/// Function dependent on selected feature index.
pub const AIN97_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8094);
/// Function dependent on selected feature index.
pub const AIN98_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8096);
/// Function dependent on selected feature index.
pub const AIN99_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8098);
/// Function dependent on selected feature index.
pub const AIN100_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8100);
/// Function dependent on selected feature index.
pub const AIN101_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8102);
/// Function dependent on selected feature index.
pub const AIN102_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8104);
/// Function dependent on selected feature index.
pub const AIN103_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8106);
/// Function dependent on selected feature index.
pub const AIN104_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8108);
/// Function dependent on selected feature index.
pub const AIN105_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8110);
/// Function dependent on selected feature index.
pub const AIN106_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8112);
/// Function dependent on selected feature index.
pub const AIN107_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8114);
/// Function dependent on selected feature index.
pub const AIN108_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8116);
/// Function dependent on selected feature index.
pub const AIN109_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8118);
/// Function dependent on selected feature index.
pub const AIN110_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8120);
/// Function dependent on selected feature index.
pub const AIN111_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8122);
/// Function dependent on selected feature index.
pub const AIN112_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8124);
/// Function dependent on selected feature index.
pub const AIN113_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8126);
/// Function dependent on selected feature index.
pub const AIN114_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8128);
/// Function dependent on selected feature index.
pub const AIN115_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8130);
/// Function dependent on selected feature index.
pub const AIN116_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8132);
/// Function dependent on selected feature index.
pub const AIN117_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8134);
/// Function dependent on selected feature index.
pub const AIN118_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8136);
/// Function dependent on selected feature index.
pub const AIN119_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8138);
/// Function dependent on selected feature index.
pub const AIN120_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8140);
/// Function dependent on selected feature index.
pub const AIN121_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8142);
/// Function dependent on selected feature index.
pub const AIN122_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8144);
/// Function dependent on selected feature index.
pub const AIN123_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8146);
/// Function dependent on selected feature index.
pub const AIN124_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8148);
/// Function dependent on selected feature index.
pub const AIN125_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8150);
/// Function dependent on selected feature index.
pub const AIN126_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8152);
/// Function dependent on selected feature index.
pub const AIN127_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8154);
/// Function dependent on selected feature index.
pub const AIN128_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8156);
/// Function dependent on selected feature index.
pub const AIN129_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8158);
/// Function dependent on selected feature index.
pub const AIN130_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8160);
/// Function dependent on selected feature index.
pub const AIN131_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8162);
/// Function dependent on selected feature index.
pub const AIN132_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8164);
/// Function dependent on selected feature index.
pub const AIN133_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8166);
/// Function dependent on selected feature index.
pub const AIN134_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8168);
/// Function dependent on selected feature index.
pub const AIN135_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8170);
/// Function dependent on selected feature index.
pub const AIN136_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8172);
/// Function dependent on selected feature index.
pub const AIN137_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8174);
/// Function dependent on selected feature index.
pub const AIN138_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8176);
/// Function dependent on selected feature index.
pub const AIN139_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8178);
/// Function dependent on selected feature index.
pub const AIN140_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8180);
/// Function dependent on selected feature index.
pub const AIN141_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8182);
/// Function dependent on selected feature index.
pub const AIN142_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8184);
/// Function dependent on selected feature index.
pub const AIN143_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8186);
/// Function dependent on selected feature index.
pub const AIN144_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8188);
/// Function dependent on selected feature index.
pub const AIN145_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8190);
/// Function dependent on selected feature index.
pub const AIN146_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8192);
/// Function dependent on selected feature index.
pub const AIN147_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8194);
/// Function dependent on selected feature index.
pub const AIN148_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8196);
/// Function dependent on selected feature index.
pub const AIN149_EF_READ_D: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8198);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN0_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8800);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN1_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8802);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN2_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8804);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN3_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8806);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN4_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8808);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN5_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8810);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN6_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8812);
/// (T8 only) Read AIN_EF with the last capture sampled, use AIN#(0:7)_EF_READ_A to read a new sample
pub const AIN7_EF_READ_A_CAPTURE: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(8814);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN0_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9000);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN1_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9002);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN2_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9004);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN3_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9006);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN4_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9008);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN5_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9010);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN6_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9012);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN7_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9014);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN8_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9016);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN9_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9018);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN10_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9020);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN11_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9022);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN12_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9024);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN13_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9026);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN14_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9028);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN15_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9030);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN16_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9032);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN17_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9034);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN18_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9036);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN19_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9038);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN20_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9040);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN21_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9042);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN22_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9044);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN23_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9046);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN24_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9048);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN25_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9050);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN26_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9052);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN27_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9054);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN28_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9056);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN29_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9058);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN30_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9060);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN31_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9062);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN32_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9064);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN33_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9066);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN34_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9068);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN35_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9070);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN36_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9072);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN37_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9074);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN38_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9076);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN39_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9078);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN40_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9080);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN41_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9082);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN42_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9084);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN43_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9086);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN44_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9088);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN45_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9090);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN46_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9092);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN47_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9094);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN48_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9096);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN49_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9098);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN50_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9100);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN51_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9102);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN52_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9104);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN53_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9106);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN54_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9108);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN55_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9110);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN56_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9112);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN57_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9114);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN58_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9116);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN59_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9118);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN60_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9120);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN61_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9122);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN62_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9124);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN63_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9126);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN64_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9128);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN65_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9130);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN66_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9132);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN67_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9134);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN68_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9136);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN69_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9138);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN70_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9140);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN71_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9142);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN72_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9144);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN73_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9146);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN74_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9148);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN75_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9150);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN76_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9152);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN77_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9154);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN78_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9156);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN79_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9158);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN80_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9160);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN81_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9162);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN82_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9164);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN83_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9166);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN84_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9168);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN85_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9170);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN86_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9172);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN87_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9174);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN88_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9176);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN89_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9178);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN90_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9180);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN91_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9182);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN92_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9184);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN93_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9186);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN94_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9188);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN95_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9190);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN96_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9192);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN97_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9194);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN98_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9196);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN99_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9198);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN100_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9200);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN101_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9202);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN102_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9204);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN103_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9206);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN104_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9208);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN105_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9210);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN106_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9212);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN107_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9214);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN108_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9216);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN109_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9218);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN110_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9220);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN111_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9222);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN112_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9224);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN113_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9226);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN114_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9228);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN115_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9230);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN116_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9232);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN117_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9234);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN118_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9236);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN119_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9238);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN120_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9240);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN121_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9242);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN122_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9244);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN123_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9246);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN124_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9248);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN125_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9250);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN126_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9252);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN127_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9254);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN128_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9256);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN129_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9258);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN130_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9260);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN131_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9262);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN132_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9264);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN133_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9266);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN134_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9268);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN135_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9270);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN136_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9272);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN137_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9274);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN138_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9276);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN139_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9278);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN140_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9280);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN141_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9282);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN142_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9284);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN143_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9286);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN144_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9288);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN145_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9290);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN146_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9292);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN147_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9294);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN148_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9296);
/// Specify the desired extended feature for this analog input with the index value. List of index values: 0=None(disabled); 1=Offset and Slope; 3=Max/Min/Avg; 4=Resistance; 5=Average and Threshold; 10=RMS Flex; 11=FlexRMS; 20=Thermocouple type E; 21=Thermocouple type J; 22=Thermocouple type K; 23=Thermocouple type R; 24=Thermocouple type T; 25=Thermocouple type S; 27=Thermocouple type N; 28=Thermocouple type B; 30=Thermocouple type C; 40=RTD model PT100; 41=RTD model PT500; 42=RTD model PT1000; 50=Thermistor Steinhart-Hart; 51=Thermistor Beta.
pub const AIN149_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9298);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9300);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9302);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9304);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9306);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9308);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9310);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9312);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9314);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9316);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9318);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9320);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9322);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9324);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9326);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9328);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9330);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9332);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9334);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9336);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9338);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9340);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9342);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9344);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9346);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9348);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9350);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9352);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9354);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9356);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9358);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9360);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9362);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9364);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9366);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9368);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9370);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9372);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9374);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9376);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9378);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9380);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9382);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9384);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9386);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9388);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9390);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9392);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9394);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9396);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9398);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9400);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9402);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9404);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9406);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9408);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9410);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9412);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9414);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9416);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9418);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9420);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9422);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9424);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9426);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9428);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9430);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9432);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9434);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9436);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9438);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9440);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9442);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9444);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9446);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9448);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9450);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9452);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9454);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9456);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9458);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9460);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9462);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9464);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9466);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9468);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9470);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9472);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9474);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9476);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9478);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9480);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9482);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9484);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9486);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9488);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9490);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9492);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9494);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9496);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9498);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9500);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9502);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9504);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9506);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9508);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9510);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9512);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9514);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9516);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9518);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9520);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9522);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9524);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9526);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9528);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9530);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9532);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9534);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9536);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9538);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9540);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9542);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9544);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9546);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9548);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9550);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9552);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9554);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9556);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9558);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9560);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9562);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9564);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9566);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9568);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9570);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9572);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9574);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9576);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9578);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9580);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9582);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9584);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9586);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9588);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9590);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9592);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9594);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9596);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9598);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9600);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9602);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9604);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9606);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9608);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9610);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9612);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9614);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9616);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9618);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9620);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9622);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9624);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9626);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9628);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9630);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9632);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9634);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9636);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9638);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9640);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9642);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9644);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9646);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9648);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9650);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9652);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9654);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9656);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9658);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9660);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9662);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9664);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9666);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9668);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9670);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9672);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9674);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9676);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9678);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9680);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9682);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9684);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9686);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9688);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9690);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9692);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9694);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9696);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9698);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9700);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9702);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9704);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9706);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9708);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9710);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9712);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9714);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9716);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9718);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9720);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9722);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9724);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9726);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9728);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9730);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9732);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9734);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9736);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9738);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9740);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9742);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9744);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9746);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9748);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9750);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9752);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9754);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9756);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9758);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9760);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9762);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9764);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9766);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9768);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9770);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9772);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9774);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9776);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9778);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9780);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9782);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9784);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9786);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9788);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9790);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9792);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9794);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9796);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9798);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9800);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9802);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9804);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9806);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9808);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9810);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9812);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9814);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9816);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9818);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9820);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9822);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9824);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9826);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9828);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9830);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9832);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9834);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9836);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9838);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9840);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9842);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9844);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9846);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9848);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9850);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9852);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9854);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9856);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9858);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9860);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9862);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9864);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9866);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9868);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9870);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9872);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9874);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9876);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9878);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9880);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9882);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9884);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9886);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9888);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9890);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9892);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9894);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9896);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9898);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9900);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9902);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9904);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9906);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9908);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9910);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9912);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9914);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9916);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9918);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9920);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9922);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9924);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9926);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9928);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9930);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9932);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9934);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9936);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9938);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9940);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9942);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9944);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9946);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9948);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9950);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9952);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9954);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9956);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9958);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9960);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9962);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9964);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9966);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9968);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9970);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9972);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9974);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9976);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9978);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9980);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9982);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9984);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9986);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9988);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9990);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9992);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9994);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9996);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(9998);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10000);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10002);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10004);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10006);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10008);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10010);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10012);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10014);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10016);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10018);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10020);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10022);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10024);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10026);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10028);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10030);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10032);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10034);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10036);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10038);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10040);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10042);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10044);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10046);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10048);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10050);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10052);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10054);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10056);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10058);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10060);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10062);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10064);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10066);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10068);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10070);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10072);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10074);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10076);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10078);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10080);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10082);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10084);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10086);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10088);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10090);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10092);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10094);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10096);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10098);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10100);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10102);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10104);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10106);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10108);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10110);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10112);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10114);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10116);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10118);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10120);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10122);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10124);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10126);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10128);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10130);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10132);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10134);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10136);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10138);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10140);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10142);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10144);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10146);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10148);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10150);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10152);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10154);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10156);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10158);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10160);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10162);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10164);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10166);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10168);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10170);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10172);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10174);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10176);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10178);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10180);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10182);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10184);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10186);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10188);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10190);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10192);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10194);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10196);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(10198);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10200);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10202);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10204);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10206);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10208);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10210);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10212);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10214);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10216);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10218);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10220);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10222);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10224);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10226);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10228);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10230);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10232);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10234);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10236);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10238);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10240);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10242);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10244);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10246);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10248);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10250);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10252);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10254);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10256);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10258);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10260);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10262);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10264);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10266);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10268);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10270);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10272);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10274);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10276);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10278);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10280);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10282);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10284);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10286);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10288);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10290);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10292);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10294);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10296);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10298);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10300);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10302);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10304);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10306);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10308);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10310);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10312);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10314);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10316);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10318);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10320);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10322);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10324);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10326);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10328);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10330);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10332);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10334);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10336);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10338);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10340);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10342);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10344);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10346);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10348);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10350);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10352);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10354);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10356);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10358);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10360);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10362);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10364);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10366);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10368);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10370);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10372);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10374);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10376);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10378);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10380);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10382);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10384);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10386);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10388);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10390);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10392);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10394);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10396);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10398);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10400);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10402);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10404);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10406);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10408);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10410);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10412);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10414);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10416);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10418);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10420);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10422);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10424);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10426);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10428);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10430);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10432);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10434);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10436);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10438);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10440);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10442);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10444);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10446);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10448);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10450);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10452);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10454);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10456);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10458);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10460);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10462);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10464);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10466);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10468);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10470);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10472);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10474);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10476);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10478);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10480);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10482);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10484);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10486);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10488);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10490);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10492);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10494);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10496);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_D: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10498);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10500);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10502);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10504);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10506);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10508);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10510);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10512);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10514);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10516);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10518);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10520);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10522);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10524);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10526);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10528);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10530);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10532);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10534);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10536);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10538);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10540);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10542);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10544);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10546);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10548);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10550);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10552);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10554);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10556);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10558);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10560);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10562);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10564);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10566);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10568);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10570);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10572);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10574);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10576);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10578);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10580);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10582);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10584);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10586);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10588);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10590);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10592);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10594);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10596);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10598);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10600);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10602);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10604);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10606);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10608);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10610);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10612);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10614);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10616);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10618);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10620);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10622);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10624);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10626);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10628);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10630);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10632);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10634);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10636);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10638);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10640);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10642);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10644);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10646);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10648);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10650);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10652);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10654);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10656);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10658);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10660);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10662);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10664);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10666);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10668);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10670);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10672);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10674);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10676);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10678);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10680);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10682);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10684);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10686);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10688);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10690);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10692);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10694);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10696);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10698);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10700);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10702);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10704);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10706);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10708);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10710);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10712);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10714);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10716);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10718);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10720);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10722);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10724);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10726);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10728);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10730);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10732);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10734);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10736);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10738);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10740);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10742);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10744);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10746);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10748);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10750);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10752);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10754);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10756);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10758);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10760);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10762);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10764);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10766);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10768);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10770);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10772);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10774);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10776);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10778);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10780);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10782);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10784);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10786);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10788);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10790);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10792);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10794);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10796);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_E: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10798);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10800);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10802);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10804);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10806);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10808);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10810);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10812);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10814);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10816);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10818);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10820);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10822);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10824);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10826);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10828);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10830);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10832);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10834);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10836);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10838);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10840);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10842);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10844);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10846);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10848);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10850);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10852);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10854);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10856);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10858);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10860);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10862);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10864);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10866);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10868);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10870);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10872);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10874);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10876);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10878);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10880);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10882);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10884);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10886);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10888);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10890);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10892);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10894);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10896);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10898);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10900);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10902);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10904);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10906);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10908);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10910);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10912);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10914);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10916);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10918);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10920);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10922);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10924);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10926);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10928);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10930);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10932);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10934);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10936);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10938);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10940);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10942);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10944);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10946);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10948);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10950);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10952);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10954);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10956);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10958);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10960);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10962);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10964);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10966);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10968);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10970);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10972);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10974);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10976);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10978);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10980);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10982);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10984);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10986);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10988);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10990);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10992);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10994);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10996);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(10998);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11000);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11002);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11004);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11006);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11008);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11010);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11012);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11014);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11016);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11018);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11020);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11022);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11024);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11026);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11028);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11030);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11032);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11034);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11036);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11038);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11040);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11042);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11044);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11046);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11048);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11050);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11052);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11054);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11056);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11058);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11060);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11062);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11064);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11066);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11068);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11070);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11072);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11074);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11076);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11078);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11080);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11082);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11084);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11086);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11088);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11090);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11092);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11094);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11096);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_F: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11098);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11100);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11102);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11104);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11106);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11108);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11110);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11112);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11114);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11116);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11118);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11120);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11122);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11124);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11126);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11128);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11130);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11132);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11134);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11136);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11138);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11140);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11142);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11144);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11146);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11148);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11150);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11152);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11154);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11156);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11158);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11160);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11162);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11164);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11166);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11168);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11170);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11172);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11174);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11176);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11178);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11180);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11182);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11184);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11186);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11188);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11190);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11192);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11194);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11196);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11198);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11200);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11202);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11204);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11206);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11208);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11210);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11212);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11214);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11216);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11218);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11220);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11222);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11224);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11226);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11228);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11230);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11232);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11234);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11236);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11238);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11240);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11242);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11244);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11246);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11248);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11250);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11252);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11254);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11256);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11258);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11260);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11262);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11264);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11266);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11268);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11270);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11272);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11274);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11276);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11278);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11280);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11282);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11284);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11286);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11288);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11290);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11292);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11294);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11296);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11298);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11300);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11302);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11304);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11306);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11308);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11310);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11312);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11314);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11316);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11318);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11320);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11322);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11324);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11326);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11328);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11330);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11332);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11334);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11336);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11338);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11340);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11342);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11344);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11346);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11348);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11350);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11352);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11354);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11356);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11358);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11360);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11362);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11364);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11366);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11368);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11370);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11372);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11374);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11376);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11378);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11380);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11382);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11384);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11386);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11388);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11390);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11392);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11394);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11396);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_G: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11398);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11400);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11402);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11404);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11406);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11408);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11410);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11412);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11414);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11416);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11418);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11420);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11422);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11424);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11426);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11428);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11430);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11432);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11434);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11436);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11438);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11440);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11442);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11444);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11446);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11448);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11450);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11452);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11454);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11456);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11458);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11460);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11462);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11464);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11466);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11468);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11470);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11472);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11474);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11476);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11478);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11480);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11482);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11484);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11486);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11488);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11490);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11492);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11494);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11496);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11498);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11500);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11502);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11504);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11506);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11508);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11510);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11512);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11514);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11516);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11518);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11520);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11522);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11524);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11526);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11528);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11530);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11532);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11534);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11536);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11538);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11540);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11542);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11544);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11546);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11548);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11550);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11552);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11554);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11556);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11558);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11560);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11562);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11564);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11566);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11568);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11570);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11572);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11574);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11576);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11578);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11580);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11582);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11584);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11586);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11588);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11590);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11592);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11594);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11596);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11598);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11600);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11602);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11604);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11606);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11608);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11610);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11612);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11614);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11616);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11618);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11620);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11622);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11624);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11626);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11628);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11630);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11632);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11634);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11636);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11638);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11640);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11642);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11644);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11646);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11648);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11650);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11652);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11654);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11656);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11658);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11660);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11662);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11664);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11666);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11668);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11670);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11672);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11674);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11676);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11678);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11680);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11682);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11684);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11686);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11688);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11690);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11692);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11694);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11696);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_H: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11698);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11700);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11702);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11704);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11706);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11708);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11710);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11712);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11714);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11716);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11718);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11720);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11722);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11724);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11726);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11728);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11730);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11732);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11734);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11736);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11738);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11740);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11742);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11744);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11746);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11748);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11750);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11752);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11754);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11756);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11758);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11760);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11762);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11764);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11766);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11768);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11770);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11772);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11774);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11776);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11778);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11780);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11782);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11784);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11786);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11788);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11790);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11792);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11794);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11796);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11798);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11800);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11802);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11804);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11806);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11808);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11810);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11812);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11814);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11816);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11818);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11820);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11822);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11824);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11826);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11828);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11830);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11832);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11834);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11836);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11838);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11840);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11842);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11844);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11846);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11848);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11850);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11852);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11854);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11856);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11858);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11860);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11862);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11864);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11866);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11868);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11870);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11872);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11874);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11876);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11878);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11880);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11882);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11884);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11886);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11888);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11890);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11892);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11894);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11896);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11898);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11900);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11902);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11904);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11906);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11908);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11910);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11912);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11914);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11916);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11918);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11920);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11922);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11924);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11926);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11928);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11930);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11932);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11934);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11936);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11938);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11940);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11942);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11944);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11946);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11948);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11950);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11952);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11954);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11956);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11958);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11960);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11962);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11964);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11966);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11968);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11970);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11972);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11974);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11976);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11978);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11980);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11982);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11984);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11986);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11988);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11990);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11992);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11994);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11996);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_I: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(11998);
/// Function dependent on selected feature index.
pub const AIN0_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12000);
/// Function dependent on selected feature index.
pub const AIN1_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12002);
/// Function dependent on selected feature index.
pub const AIN2_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12004);
/// Function dependent on selected feature index.
pub const AIN3_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12006);
/// Function dependent on selected feature index.
pub const AIN4_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12008);
/// Function dependent on selected feature index.
pub const AIN5_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12010);
/// Function dependent on selected feature index.
pub const AIN6_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12012);
/// Function dependent on selected feature index.
pub const AIN7_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12014);
/// Function dependent on selected feature index.
pub const AIN8_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12016);
/// Function dependent on selected feature index.
pub const AIN9_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12018);
/// Function dependent on selected feature index.
pub const AIN10_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12020);
/// Function dependent on selected feature index.
pub const AIN11_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12022);
/// Function dependent on selected feature index.
pub const AIN12_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12024);
/// Function dependent on selected feature index.
pub const AIN13_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12026);
/// Function dependent on selected feature index.
pub const AIN14_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12028);
/// Function dependent on selected feature index.
pub const AIN15_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12030);
/// Function dependent on selected feature index.
pub const AIN16_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12032);
/// Function dependent on selected feature index.
pub const AIN17_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12034);
/// Function dependent on selected feature index.
pub const AIN18_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12036);
/// Function dependent on selected feature index.
pub const AIN19_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12038);
/// Function dependent on selected feature index.
pub const AIN20_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12040);
/// Function dependent on selected feature index.
pub const AIN21_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12042);
/// Function dependent on selected feature index.
pub const AIN22_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12044);
/// Function dependent on selected feature index.
pub const AIN23_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12046);
/// Function dependent on selected feature index.
pub const AIN24_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12048);
/// Function dependent on selected feature index.
pub const AIN25_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12050);
/// Function dependent on selected feature index.
pub const AIN26_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12052);
/// Function dependent on selected feature index.
pub const AIN27_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12054);
/// Function dependent on selected feature index.
pub const AIN28_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12056);
/// Function dependent on selected feature index.
pub const AIN29_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12058);
/// Function dependent on selected feature index.
pub const AIN30_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12060);
/// Function dependent on selected feature index.
pub const AIN31_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12062);
/// Function dependent on selected feature index.
pub const AIN32_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12064);
/// Function dependent on selected feature index.
pub const AIN33_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12066);
/// Function dependent on selected feature index.
pub const AIN34_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12068);
/// Function dependent on selected feature index.
pub const AIN35_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12070);
/// Function dependent on selected feature index.
pub const AIN36_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12072);
/// Function dependent on selected feature index.
pub const AIN37_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12074);
/// Function dependent on selected feature index.
pub const AIN38_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12076);
/// Function dependent on selected feature index.
pub const AIN39_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12078);
/// Function dependent on selected feature index.
pub const AIN40_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12080);
/// Function dependent on selected feature index.
pub const AIN41_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12082);
/// Function dependent on selected feature index.
pub const AIN42_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12084);
/// Function dependent on selected feature index.
pub const AIN43_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12086);
/// Function dependent on selected feature index.
pub const AIN44_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12088);
/// Function dependent on selected feature index.
pub const AIN45_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12090);
/// Function dependent on selected feature index.
pub const AIN46_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12092);
/// Function dependent on selected feature index.
pub const AIN47_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12094);
/// Function dependent on selected feature index.
pub const AIN48_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12096);
/// Function dependent on selected feature index.
pub const AIN49_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12098);
/// Function dependent on selected feature index.
pub const AIN50_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12100);
/// Function dependent on selected feature index.
pub const AIN51_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12102);
/// Function dependent on selected feature index.
pub const AIN52_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12104);
/// Function dependent on selected feature index.
pub const AIN53_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12106);
/// Function dependent on selected feature index.
pub const AIN54_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12108);
/// Function dependent on selected feature index.
pub const AIN55_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12110);
/// Function dependent on selected feature index.
pub const AIN56_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12112);
/// Function dependent on selected feature index.
pub const AIN57_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12114);
/// Function dependent on selected feature index.
pub const AIN58_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12116);
/// Function dependent on selected feature index.
pub const AIN59_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12118);
/// Function dependent on selected feature index.
pub const AIN60_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12120);
/// Function dependent on selected feature index.
pub const AIN61_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12122);
/// Function dependent on selected feature index.
pub const AIN62_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12124);
/// Function dependent on selected feature index.
pub const AIN63_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12126);
/// Function dependent on selected feature index.
pub const AIN64_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12128);
/// Function dependent on selected feature index.
pub const AIN65_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12130);
/// Function dependent on selected feature index.
pub const AIN66_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12132);
/// Function dependent on selected feature index.
pub const AIN67_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12134);
/// Function dependent on selected feature index.
pub const AIN68_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12136);
/// Function dependent on selected feature index.
pub const AIN69_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12138);
/// Function dependent on selected feature index.
pub const AIN70_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12140);
/// Function dependent on selected feature index.
pub const AIN71_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12142);
/// Function dependent on selected feature index.
pub const AIN72_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12144);
/// Function dependent on selected feature index.
pub const AIN73_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12146);
/// Function dependent on selected feature index.
pub const AIN74_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12148);
/// Function dependent on selected feature index.
pub const AIN75_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12150);
/// Function dependent on selected feature index.
pub const AIN76_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12152);
/// Function dependent on selected feature index.
pub const AIN77_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12154);
/// Function dependent on selected feature index.
pub const AIN78_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12156);
/// Function dependent on selected feature index.
pub const AIN79_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12158);
/// Function dependent on selected feature index.
pub const AIN80_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12160);
/// Function dependent on selected feature index.
pub const AIN81_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12162);
/// Function dependent on selected feature index.
pub const AIN82_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12164);
/// Function dependent on selected feature index.
pub const AIN83_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12166);
/// Function dependent on selected feature index.
pub const AIN84_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12168);
/// Function dependent on selected feature index.
pub const AIN85_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12170);
/// Function dependent on selected feature index.
pub const AIN86_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12172);
/// Function dependent on selected feature index.
pub const AIN87_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12174);
/// Function dependent on selected feature index.
pub const AIN88_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12176);
/// Function dependent on selected feature index.
pub const AIN89_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12178);
/// Function dependent on selected feature index.
pub const AIN90_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12180);
/// Function dependent on selected feature index.
pub const AIN91_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12182);
/// Function dependent on selected feature index.
pub const AIN92_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12184);
/// Function dependent on selected feature index.
pub const AIN93_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12186);
/// Function dependent on selected feature index.
pub const AIN94_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12188);
/// Function dependent on selected feature index.
pub const AIN95_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12190);
/// Function dependent on selected feature index.
pub const AIN96_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12192);
/// Function dependent on selected feature index.
pub const AIN97_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12194);
/// Function dependent on selected feature index.
pub const AIN98_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12196);
/// Function dependent on selected feature index.
pub const AIN99_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12198);
/// Function dependent on selected feature index.
pub const AIN100_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12200);
/// Function dependent on selected feature index.
pub const AIN101_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12202);
/// Function dependent on selected feature index.
pub const AIN102_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12204);
/// Function dependent on selected feature index.
pub const AIN103_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12206);
/// Function dependent on selected feature index.
pub const AIN104_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12208);
/// Function dependent on selected feature index.
pub const AIN105_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12210);
/// Function dependent on selected feature index.
pub const AIN106_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12212);
/// Function dependent on selected feature index.
pub const AIN107_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12214);
/// Function dependent on selected feature index.
pub const AIN108_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12216);
/// Function dependent on selected feature index.
pub const AIN109_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12218);
/// Function dependent on selected feature index.
pub const AIN110_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12220);
/// Function dependent on selected feature index.
pub const AIN111_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12222);
/// Function dependent on selected feature index.
pub const AIN112_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12224);
/// Function dependent on selected feature index.
pub const AIN113_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12226);
/// Function dependent on selected feature index.
pub const AIN114_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12228);
/// Function dependent on selected feature index.
pub const AIN115_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12230);
/// Function dependent on selected feature index.
pub const AIN116_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12232);
/// Function dependent on selected feature index.
pub const AIN117_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12234);
/// Function dependent on selected feature index.
pub const AIN118_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12236);
/// Function dependent on selected feature index.
pub const AIN119_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12238);
/// Function dependent on selected feature index.
pub const AIN120_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12240);
/// Function dependent on selected feature index.
pub const AIN121_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12242);
/// Function dependent on selected feature index.
pub const AIN122_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12244);
/// Function dependent on selected feature index.
pub const AIN123_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12246);
/// Function dependent on selected feature index.
pub const AIN124_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12248);
/// Function dependent on selected feature index.
pub const AIN125_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12250);
/// Function dependent on selected feature index.
pub const AIN126_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12252);
/// Function dependent on selected feature index.
pub const AIN127_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12254);
/// Function dependent on selected feature index.
pub const AIN128_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12256);
/// Function dependent on selected feature index.
pub const AIN129_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12258);
/// Function dependent on selected feature index.
pub const AIN130_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12260);
/// Function dependent on selected feature index.
pub const AIN131_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12262);
/// Function dependent on selected feature index.
pub const AIN132_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12264);
/// Function dependent on selected feature index.
pub const AIN133_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12266);
/// Function dependent on selected feature index.
pub const AIN134_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12268);
/// Function dependent on selected feature index.
pub const AIN135_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12270);
/// Function dependent on selected feature index.
pub const AIN136_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12272);
/// Function dependent on selected feature index.
pub const AIN137_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12274);
/// Function dependent on selected feature index.
pub const AIN138_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12276);
/// Function dependent on selected feature index.
pub const AIN139_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12278);
/// Function dependent on selected feature index.
pub const AIN140_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12280);
/// Function dependent on selected feature index.
pub const AIN141_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12282);
/// Function dependent on selected feature index.
pub const AIN142_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12284);
/// Function dependent on selected feature index.
pub const AIN143_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12286);
/// Function dependent on selected feature index.
pub const AIN144_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12288);
/// Function dependent on selected feature index.
pub const AIN145_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12290);
/// Function dependent on selected feature index.
pub const AIN146_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12292);
/// Function dependent on selected feature index.
pub const AIN147_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12294);
/// Function dependent on selected feature index.
pub const AIN148_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12296);
/// Function dependent on selected feature index.
pub const AIN149_EF_CONFIG_J: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(12298);
/// Get the latest temperature conversion, and begin a new conversion. Starting with FW1.21, temperature conversion is instantaneous. Units are described in TMP112 datasheet.
pub const DGT_TEMPERATURE_LATEST_RAW: LabjackTag<u16, CanRead, CannotWrite> =
    LabjackTag::new(22000);
/// Craw. Get a new humidity reading from HCH-1000. Units are in 100s fF of capacitance.
pub const DGT_HUMIDITY_RAW: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22001);
/// Counts. Get a new light reading from LED. Units are in counts before reverse leakage complete.
pub const DGT_LIGHT_RAW: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22002);
/// Set the items to log. Value is a bitmask: Bit0: 1 = Log Temperature, Bit1: 1 = Log Light, Bit2: 1 = Log Humidity. All other bits reserved. Change implemented on next erase of dataset.
pub const DGT_CHANGE_LOG_ITEMS: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22018);
/// Read what is logged in the dataset. Value is a bitmask: Bit0: 1 = Temperature logged, Bit1: 1 = Light logged, Bit2: 1 = Humidity logged. In memory: T, TL, TH, THL.
pub const DGT_LOG_ITEMS_DATASET: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22019);
/// Set the time interval between log events. Index 0=10s, 1=30s, 2=1m, 3=10m, 4=30m, 5=1h, 6=6h Change implemented on next erase of dataset.
pub const DGT_CHANGE_LOG_INTERVAL_INDEX: LabjackTag<u16, CanRead, CanWrite> =
    LabjackTag::new(22038);
/// Read time interval between log events in the dataset. Index 0=10s, 1=30s, 2=1m, 3=10m, 4=30m, 5=1h, 6=6h
pub const DGT_LOG_INTERVAL_INDEX_DATASET: LabjackTag<u16, CanRead, CannotWrite> =
    LabjackTag::new(22039);
/// The start time of the logging session: 22042 Year=0 to 99, 22043 Month=1 to 12, 22044 Day=1 to 31, 22045 Weekday=1 to 7, 22046 Hour=0 to 23, 22047 Minutes=0 to 59, 22048 Second=0 to 59
pub const DGT_LOG_START_TIME: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22042);
/// A bitmask of all alarm and error flags. 0=Occurred, 1=Did NOT occur. Refer to documentation for more information
pub const DGT_NALARM_FLAGS: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22049);
/// User high threshold alarm triggered. 0=Occurred, 1=Did NOT occur
pub const DGT_NTRIG_HIGH_ALARM: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22053);
/// User low threshold alarm triggered. 0=Occurred, 1=Did NOT occur
pub const DGT_NTRIG_LOW_ALARM: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22054);
/// User alarms armed flag. 0=Armed, 1=Not yet armed
pub const DGT_NALARM_ARMED: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22055);
/// Index of the user alarm item. 0=None, 1=Temperature, 2=Light, 3=Humidity. Valid results require that the corresponding item is being logged.
pub const DGT_INDEX_ALARM_ITEM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22080);
/// Index of the alarm arming condition. 0=Arm once within threshold window, 1=Arm after pure time delay
pub const DGT_INDEX_ALARM_ARM_CONDITION: LabjackTag<u16, CanRead, CanWrite> =
    LabjackTag::new(22081);
/// User high threshold alarm enable. 0=Disabled, 1=Enabled
pub const DGT_HIGH_ALARM_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22082);
/// User low threshold alarm enable. 0=Disabled, 1=Enabled
pub const DGT_LOW_ALARM_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22083);
/// User threshold alarm arming delay counter. The number of consecutive readings (inside window) before arming the alarm-> 0 to 63
pub const DGT_ALARM_ARM_DELAY: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22084);
/// User threshold alarm trigger delay counter. The number of consecutive readings outside window before triggering the alarm-> 0 to 31
pub const DGT_ALARM_TRIG_DELAY: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22085);
/// User alarm high threshold value. Value depends on which item is being alarmed on(T, L, or H).
pub const DGT_HIGH_ALARM_THRESHOLD: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22100);
/// User alarm low threshold value. Value depends on which item is being alarmed on(T, L, or H).
pub const DGT_LOW_ALARM_THRESHOLD: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22101);
/// High light threshold pointer(because light alarms require an array of count values)-> 0 to 255. Auto-incremented
pub const DGT_PHIGH_THRESH_LIGHT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22180);
/// High light threshold array start.
pub const DGT_HIGH_THRESH_LIGHT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22181);
/// Low light threshold pointer(because light alarms require an array of count values)-> 0 to 255. Auto-incremented
pub const DGT_PLOW_THRESH_LIGHT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22190);
/// Low light threshold array start.
pub const DGT_LOW_THRESH_LIGHT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22191);
/// Number of things installed on the Digit 2=TL, 3=TLH
pub const DGT_INSTALLED_OPTIONS: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22200);
/// Number of bytes of data stored on flash.
pub const DGT_STORED_BYTES: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(22210);
/// Total number of readings saved since last cleared. Typically cleared if new battery is installed.
pub const DGT_LIFETIME_READINGS: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(22220);
/// The date that the battery was installed. Timestamp format = Seconds since 1970. This parameter is used to estimate remaining battery life.
pub const DGT_BATTERY_INSTALL_DATE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(22222);
/// The factory calibration offset which can be used in the conversion of raw humidity. Write requires unlocked.
pub const DGT_HUMIDITY_CAL_OFFSET_FACTORY: LabjackTag<u16, CanRead, CanWrite> =
    LabjackTag::new(22231);
/// A single point offset used in the conversion of raw humidity. Units in 100s fF
pub const DGT_HUMIDITY_CAL_OFFSET: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22232);
/// Read=Constant current source trim value. Write(any value)=Automatically calibrate the current source. The current source is used for humidity readings, and once calibrated, it will not need to be calibrated again.
pub const DGT_HUMIDITY_CAL_I_SOURCE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22233);
/// The factory slope of humidity which can be used in the conversion of raw humidity. Units in %Rh per 100s fF. Write requires unlocked.
pub const DGT_HUMIDITY_CAL_SLOPE_FACTORY: LabjackTag<f32, CanRead, CanWrite> =
    LabjackTag::new(22234);
/// The slope of humidity used in the conversion of raw humidity. Units in %Rh per 100s fF.
pub const DGT_HUMIDITY_CAL_SLOPE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(22236);
/// The temperature dependent factor used in the conversion of raw humidity. Units in 100s fF per degC
pub const DGT_HUMIDITY_CAL_T_SLOPE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(22238);
/// The factory temperature dependent factor used in the conversion of raw humidity. Units in 100s fF per degC
pub const DGT_HUMIDITY_CAL_T_SLOPE_FACTORY: LabjackTag<f32, CanRead, CanWrite> =
    LabjackTag::new(22240);
/// The current time of the real-time-clock on the Digit. Synchronized with the computer within software. 22242 Year=0 to 99, 22243 Month=1 to 12, 22244 Day=1 to 31, 22245 Weekday=1 to 7, 22246 Hour=0 to 23, 22247 Minutes=0 to 59, 22248 Second=0 to 59
pub const DGT_RTCC_TIME: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(22242);
/// A status flag for the flash(data storage chip) on the Digit. 1=Write in-progress, 0=Idle
pub const DGT_FLASH_WIP: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22804);
/// The read pointer for flash. Auto-incremented, simply initialize with a start address, then use 22812+
pub const DGT_PFLASH_READ: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(22810);
/// Read data from this address and subsequent addresses after pointer is initialized. Used to download data from the Digit.
pub const DGT_FLASH_READ: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(22812);
/// Erase all data on the flash. KEY=Erase, Other=No effect, KEY=37157 Takes 5 seconds to complete, so poll the WIP status flag until =0. After erase, logging will begin immediately.
pub const DGT_FLASH_BKERASE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(22822);
/// The write pointer for flash. Auto-incremented, simply initialize with a start address, then use 22832+
pub const DGT_PFLASH_WRITE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(22830);
/// Write data to this address and subsequent addresses after pointer is initialized. Used primarily for firmware updates.
pub const DGT_FLASH_WRITE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(22832);
/// Write a unique passkey to unlock factory settings. Only used by mfr.
pub const DGT_FACTORY_UNLOCK: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(22844);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC0: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30000);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC1: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30002);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC2: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30004);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC3: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30006);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC4: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30008);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC5: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30010);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC6: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30012);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC7: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30014);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC8: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30016);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC9: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30018);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC10: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30020);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC11: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30022);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC12: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30024);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC13: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30026);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC14: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30028);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC15: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30030);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC16: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30032);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC17: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30034);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC18: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30036);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC19: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30038);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC20: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30040);
/// Update a voltage output on a connected LJTick-DAC accessory. Even TDAC# = DACA, Odd TDAC# = DACB. For instance, if LJTick-DAC accessory is connected to FIO2/FIO3 block on main device, TDAC2 corresponds with DACA, and TDAC3 corresponds with DACB.
pub const TDAC21: LabjackTag<f32, CannotRead, CanWrite> = LabjackTag::new(30042);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS0_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30100);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS1_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30102);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS2_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30104);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS3_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30106);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS4_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30108);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS5_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30110);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS6_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30112);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS7_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30114);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS8_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30116);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS9_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30118);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS10_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30120);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS11_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30122);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS12_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30124);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS13_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30126);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS14_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30128);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS15_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30130);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS16_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30132);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS17_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30134);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS18_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30136);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS19_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30138);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS20_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30140);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS21_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30142);
/// Reads temperature in Kelvin from an SBUS sensor (EI-1050/SHT1x/SHT7x).  SBUS# is the DIO line for the EI-1050 enable.  If SBUS# is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS22_TEMP: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30144);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS0_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30150);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS1_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30152);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS2_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30154);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS3_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30156);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS4_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30158);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS5_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30160);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS6_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30162);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS7_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30164);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS8_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30166);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS9_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30168);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS10_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30170);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS11_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30172);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS12_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30174);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS13_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30176);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS14_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30178);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS15_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30180);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS16_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30182);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS17_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30184);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS18_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30186);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS19_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30188);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS20_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30190);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS21_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30192);
/// Reads humidity in % from an external SBUS sensor (EI-1050/SHT1x/SHT7x).  # is the DIO line for the EI-1050 enable.  If # is the same as the value specified for data or clock line, there will be no control of an enable line.
pub const SBUS22_RH: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(30194);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS0_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30200);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS1_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30201);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS2_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30202);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS3_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30203);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS4_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30204);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS5_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30205);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS6_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30206);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS7_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30207);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS8_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30208);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS9_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30209);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS10_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30210);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS11_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30211);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS12_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30212);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS13_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30213);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS14_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30214);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS15_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30215);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS16_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30216);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS17_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30217);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS18_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30218);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS19_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30219);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS20_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30220);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS21_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30221);
/// This is the DIO# that the external sensor's data line is connected to.
pub const SBUS22_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30222);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS0_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30225);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS1_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30226);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS2_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30227);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS3_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30228);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS4_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30229);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS5_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30230);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS6_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30231);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS7_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30232);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS8_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30233);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS9_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30234);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS10_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30235);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS11_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30236);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS12_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30237);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS13_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30238);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS14_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30239);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS15_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30240);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS16_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30241);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS17_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30242);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS18_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30243);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS19_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30244);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS20_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30245);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS21_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30246);
/// This is the DIO# that the external sensor's clock line is connected to.
pub const SBUS22_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30247);
/// Currently unsupported.
pub const SBUS0_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30250);
/// Currently unsupported.
pub const SBUS1_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30251);
/// Currently unsupported.
pub const SBUS2_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30252);
/// Currently unsupported.
pub const SBUS3_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30253);
/// Currently unsupported.
pub const SBUS4_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30254);
/// Currently unsupported.
pub const SBUS5_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30255);
/// Currently unsupported.
pub const SBUS6_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30256);
/// Currently unsupported.
pub const SBUS7_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30257);
/// Currently unsupported.
pub const SBUS8_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30258);
/// Currently unsupported.
pub const SBUS9_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30259);
/// Currently unsupported.
pub const SBUS10_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30260);
/// Currently unsupported.
pub const SBUS11_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30261);
/// Currently unsupported.
pub const SBUS12_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30262);
/// Currently unsupported.
pub const SBUS13_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30263);
/// Currently unsupported.
pub const SBUS14_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30264);
/// Currently unsupported.
pub const SBUS15_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30265);
/// Currently unsupported.
pub const SBUS16_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30266);
/// Currently unsupported.
pub const SBUS17_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30267);
/// Currently unsupported.
pub const SBUS18_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30268);
/// Currently unsupported.
pub const SBUS19_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30269);
/// Currently unsupported.
pub const SBUS20_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30270);
/// Currently unsupported.
pub const SBUS21_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30271);
/// Currently unsupported.
pub const SBUS22_BACKGROUND_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30272);
/// A write to this global parameter sets all SBUS data line registers to the same value. A read will return the correct setting if all channels are set the same, but otherwise will return 0xFF.
pub const SBUS_ALL_DATA_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30275);
/// A write to this global parameter sets all SBUS clock line registers to the same value. A read will return the correct setting if all channels are set the same, but otherwise will return 0xFF.
pub const SBUS_ALL_CLOCK_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30276);
/// Sets the power line. This DIO is set to output-high upon any read of SBUS#_TEMP or SBUS#_RH. Default is FIO6 for the T4 and FIO2 for the T7. An FIO line can power up to 4 sensors while an EIO/CIO/MIO line or DAC line can power up to 20 sensors. Set to 9999 to disable. To use multiple power lines, use a DAC line for power, or otherwise control power yourself, set this to 9999 and then control power using writes to normal registers such as FIO5, EIO0, or DAC0.
pub const SBUS_ALL_POWER_DIONUM: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30277);
/// Sets the clock speed. The clock is software generated so the resulting frequency is not exact. Valid range is 0-65535. Larger values are faster. 0 is the fastest option and is equivalent to 65536. A value of 0 is ~200 kHz. A value of 65000 is ~9.1 kHz.
pub const SBUS_ALL_CLOCK_SPEED: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(30278);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN0_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40000);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN1_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40002);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN2_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40004);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN3_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40006);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN4_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40008);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN5_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40010);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN6_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40012);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN7_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40014);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN8_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40016);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN9_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40018);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN10_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40020);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN11_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40022);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN12_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40024);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN13_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40026);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN14_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40028);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN15_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40030);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN16_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40032);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN17_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40034);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN18_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40036);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN19_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40038);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN20_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40040);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN21_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40042);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN22_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40044);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN23_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40046);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN24_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40048);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN25_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40050);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN26_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40052);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN27_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40054);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN28_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40056);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN29_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40058);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN30_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40060);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN31_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40062);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN32_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40064);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN33_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40066);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN34_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40068);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN35_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40070);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN36_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40072);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN37_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40074);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN38_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40076);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN39_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40078);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN40_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40080);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN41_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40082);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN42_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40084);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN43_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40086);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN44_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40088);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN45_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40090);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN46_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40092);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN47_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40094);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN48_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40096);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN49_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40098);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN50_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40100);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN51_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40102);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN52_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40104);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN53_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40106);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN54_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40108);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN55_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40110);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN56_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40112);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN57_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40114);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN58_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40116);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN59_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40118);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN60_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40120);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN61_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40122);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN62_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40124);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN63_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40126);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN64_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40128);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN65_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40130);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN66_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40132);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN67_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40134);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN68_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40136);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN69_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40138);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN70_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40140);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN71_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40142);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN72_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40144);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN73_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40146);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN74_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40148);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN75_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40150);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN76_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40152);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN77_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40154);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN78_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40156);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN79_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40158);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN80_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40160);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN81_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40162);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN82_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40164);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN83_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40166);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN84_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40168);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN85_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40170);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN86_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40172);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN87_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40174);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN88_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40176);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN89_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40178);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN90_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40180);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN91_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40182);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN92_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40184);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN93_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40186);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN94_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40188);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN95_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40190);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN96_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40192);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN97_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40194);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN98_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40196);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN99_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40198);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN100_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40200);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN101_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40202);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN102_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40204);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN103_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40206);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN104_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40208);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN105_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40210);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN106_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40212);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN107_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40214);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN108_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40216);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN109_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40218);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN110_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40220);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN111_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40222);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN112_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40224);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN113_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40226);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN114_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40228);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN115_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40230);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN116_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40232);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN117_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40234);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN118_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40236);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN119_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40238);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN120_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40240);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN121_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40242);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN122_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40244);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN123_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40246);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN124_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40248);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN125_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40250);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN126_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40252);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN127_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40254);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN128_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40256);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN129_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40258);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN130_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40260);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN131_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40262);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN132_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40264);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN133_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40266);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN134_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40268);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN135_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40270);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN136_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40272);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN137_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40274);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN138_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40276);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN139_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40278);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN140_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40280);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN141_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40282);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN142_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40284);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN143_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40286);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN144_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40288);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN145_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40290);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN146_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40292);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN147_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40294);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN148_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40296);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN149_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40298);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN150_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40300);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN151_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40302);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN152_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40304);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN153_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40306);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN154_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40308);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN155_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40310);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN156_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40312);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN157_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40314);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN158_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40316);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN159_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40318);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN160_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40320);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN161_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40322);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN162_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40324);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN163_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40326);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN164_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40328);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN165_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40330);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN166_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40332);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN167_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40334);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN168_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40336);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN169_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40338);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN170_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40340);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN171_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40342);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN172_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40344);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN173_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40346);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN174_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40348);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN175_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40350);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN176_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40352);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN177_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40354);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN178_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40356);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN179_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40358);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN180_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40360);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN181_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40362);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN182_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40364);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN183_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40366);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN184_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40368);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN185_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40370);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN186_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40372);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN187_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40374);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN188_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40376);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN189_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40378);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN190_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40380);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN191_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40382);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN192_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40384);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN193_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40386);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN194_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40388);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN195_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40390);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN196_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40392);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN197_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40394);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN198_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40396);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN199_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40398);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN200_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40400);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN201_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40402);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN202_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40404);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN203_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40406);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN204_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40408);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN205_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40410);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN206_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40412);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN207_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40414);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN208_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40416);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN209_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40418);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN210_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40420);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN211_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40422);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN212_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40424);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN213_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40426);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN214_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40428);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN215_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40430);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN216_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40432);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN217_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40434);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN218_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40436);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN219_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40438);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN220_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40440);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN221_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40442);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN222_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40444);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN223_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40446);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN224_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40448);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN225_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40450);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN226_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40452);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN227_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40454);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN228_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40456);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN229_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40458);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN230_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40460);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN231_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40462);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN232_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40464);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN233_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40466);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN234_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40468);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN235_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40470);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN236_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40472);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN237_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40474);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN238_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40476);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN239_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40478);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN240_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40480);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN241_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40482);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN242_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40484);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN243_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40486);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN244_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40488);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN245_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40490);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN246_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40492);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN247_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40494);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN248_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40496);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN249_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40498);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN250_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40500);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN251_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40502);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN252_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40504);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN253_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40506);
/// The range/span of a single analog input. Select the desired range by writing a value from the device specific list.
pub const AIN254_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(40508);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN0_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41000);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN1_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41001);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN2_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41002);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN3_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41003);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN4_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41004);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN5_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41005);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN6_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41006);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN7_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41007);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN8_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41008);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN9_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41009);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN10_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41010);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN11_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41011);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN12_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41012);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN13_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41013);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN14_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41014);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN15_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41015);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN16_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41016);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN17_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41017);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN18_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41018);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN19_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41019);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN20_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41020);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN21_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41021);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN22_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41022);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN23_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41023);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN24_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41024);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN25_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41025);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN26_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41026);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN27_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41027);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN28_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41028);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN29_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41029);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN30_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41030);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN31_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41031);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN32_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41032);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN33_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41033);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN34_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41034);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN35_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41035);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN36_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41036);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN37_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41037);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN38_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41038);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN39_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41039);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN40_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41040);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN41_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41041);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN42_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41042);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN43_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41043);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN44_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41044);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN45_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41045);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN46_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41046);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN47_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41047);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN48_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41048);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN49_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41049);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN50_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41050);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN51_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41051);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN52_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41052);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN53_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41053);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN54_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41054);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN55_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41055);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN56_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41056);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN57_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41057);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN58_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41058);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN59_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41059);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN60_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41060);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN61_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41061);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN62_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41062);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN63_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41063);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN64_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41064);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN65_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41065);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN66_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41066);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN67_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41067);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN68_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41068);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN69_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41069);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN70_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41070);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN71_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41071);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN72_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41072);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN73_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41073);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN74_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41074);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN75_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41075);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN76_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41076);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN77_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41077);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN78_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41078);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN79_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41079);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN80_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41080);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN81_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41081);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN82_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41082);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN83_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41083);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN84_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41084);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN85_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41085);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN86_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41086);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN87_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41087);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN88_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41088);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN89_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41089);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN90_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41090);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN91_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41091);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN92_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41092);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN93_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41093);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN94_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41094);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN95_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41095);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN96_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41096);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN97_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41097);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN98_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41098);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN99_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41099);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN100_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41100);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN101_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41101);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN102_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41102);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN103_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41103);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN104_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41104);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN105_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41105);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN106_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41106);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN107_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41107);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN108_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41108);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN109_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41109);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN110_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41110);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN111_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41111);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN112_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41112);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN113_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41113);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN114_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41114);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN115_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41115);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN116_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41116);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN117_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41117);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN118_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41118);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN119_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41119);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN120_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41120);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN121_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41121);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN122_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41122);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN123_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41123);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN124_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41124);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN125_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41125);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN126_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41126);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN127_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41127);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN128_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41128);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN129_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41129);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN130_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41130);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN131_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41131);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN132_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41132);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN133_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41133);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN134_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41134);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN135_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41135);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN136_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41136);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN137_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41137);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN138_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41138);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN139_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41139);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN140_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41140);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN141_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41141);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN142_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41142);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN143_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41143);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN144_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41144);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN145_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41145);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN146_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41146);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN147_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41147);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN148_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41148);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN149_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41149);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN150_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41150);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN151_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41151);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN152_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41152);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN153_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41153);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN154_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41154);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN155_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41155);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN156_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41156);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN157_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41157);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN158_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41158);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN159_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41159);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN160_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41160);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN161_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41161);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN162_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41162);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN163_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41163);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN164_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41164);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN165_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41165);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN166_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41166);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN167_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41167);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN168_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41168);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN169_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41169);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN170_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41170);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN171_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41171);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN172_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41172);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN173_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41173);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN174_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41174);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN175_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41175);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN176_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41176);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN177_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41177);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN178_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41178);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN179_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41179);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN180_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41180);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN181_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41181);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN182_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41182);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN183_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41183);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN184_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41184);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN185_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41185);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN186_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41186);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN187_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41187);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN188_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41188);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN189_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41189);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN190_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41190);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN191_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41191);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN192_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41192);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN193_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41193);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN194_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41194);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN195_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41195);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN196_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41196);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN197_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41197);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN198_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41198);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN199_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41199);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN200_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41200);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN201_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41201);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN202_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41202);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN203_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41203);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN204_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41204);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN205_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41205);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN206_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41206);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN207_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41207);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN208_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41208);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN209_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41209);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN210_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41210);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN211_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41211);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN212_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41212);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN213_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41213);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN214_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41214);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN215_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41215);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN216_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41216);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN217_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41217);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN218_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41218);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN219_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41219);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN220_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41220);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN221_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41221);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN222_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41222);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN223_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41223);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN224_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41224);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN225_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41225);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN226_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41226);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN227_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41227);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN228_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41228);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN229_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41229);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN230_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41230);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN231_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41231);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN232_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41232);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN233_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41233);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN234_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41234);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN235_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41235);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN236_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41236);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN237_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41237);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN238_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41238);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN239_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41239);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN240_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41240);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN241_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41241);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN242_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41242);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN243_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41243);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN244_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41244);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN245_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41245);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN246_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41246);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN247_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41247);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN248_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41248);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN249_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41249);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN250_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41250);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN251_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41251);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN252_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41252);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN253_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41253);
/// Specifies the negative channel to be used for each positive channel. 199=Default=> Single-Ended.
pub const AIN254_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41254);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN0_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41500);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN1_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41501);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN2_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41502);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN3_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41503);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN4_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41504);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN5_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41505);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN6_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41506);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN7_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41507);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN8_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41508);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN9_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41509);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN10_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41510);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN11_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41511);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN12_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41512);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN13_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41513);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN14_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41514);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN15_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41515);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN16_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41516);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN17_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41517);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN18_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41518);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN19_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41519);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN20_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41520);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN21_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41521);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN22_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41522);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN23_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41523);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN24_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41524);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN25_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41525);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN26_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41526);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN27_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41527);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN28_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41528);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN29_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41529);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN30_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41530);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN31_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41531);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN32_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41532);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN33_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41533);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN34_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41534);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN35_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41535);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN36_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41536);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN37_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41537);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN38_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41538);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN39_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41539);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN40_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41540);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN41_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41541);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN42_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41542);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN43_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41543);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN44_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41544);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN45_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41545);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN46_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41546);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN47_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41547);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN48_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41548);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN49_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41549);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN50_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41550);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN51_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41551);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN52_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41552);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN53_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41553);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN54_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41554);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN55_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41555);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN56_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41556);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN57_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41557);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN58_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41558);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN59_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41559);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN60_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41560);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN61_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41561);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN62_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41562);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN63_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41563);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN64_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41564);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN65_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41565);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN66_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41566);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN67_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41567);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN68_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41568);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN69_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41569);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN70_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41570);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN71_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41571);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN72_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41572);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN73_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41573);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN74_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41574);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN75_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41575);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN76_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41576);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN77_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41577);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN78_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41578);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN79_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41579);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN80_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41580);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN81_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41581);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN82_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41582);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN83_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41583);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN84_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41584);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN85_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41585);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN86_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41586);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN87_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41587);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN88_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41588);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN89_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41589);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN90_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41590);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN91_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41591);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN92_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41592);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN93_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41593);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN94_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41594);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN95_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41595);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN96_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41596);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN97_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41597);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN98_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41598);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN99_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41599);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN100_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41600);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN101_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41601);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN102_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41602);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN103_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41603);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN104_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41604);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN105_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41605);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN106_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41606);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN107_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41607);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN108_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41608);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN109_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41609);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN110_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41610);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN111_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41611);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN112_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41612);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN113_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41613);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN114_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41614);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN115_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41615);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN116_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41616);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN117_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41617);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN118_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41618);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN119_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41619);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN120_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41620);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN121_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41621);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN122_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41622);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN123_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41623);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN124_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41624);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN125_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41625);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN126_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41626);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN127_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41627);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN128_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41628);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN129_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41629);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN130_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41630);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN131_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41631);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN132_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41632);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN133_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41633);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN134_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41634);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN135_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41635);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN136_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41636);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN137_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41637);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN138_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41638);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN139_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41639);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN140_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41640);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN141_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41641);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN142_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41642);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN143_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41643);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN144_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41644);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN145_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41645);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN146_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41646);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN147_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41647);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN148_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41648);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN149_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41649);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN150_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41650);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN151_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41651);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN152_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41652);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN153_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41653);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN154_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41654);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN155_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41655);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN156_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41656);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN157_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41657);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN158_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41658);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN159_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41659);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN160_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41660);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN161_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41661);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN162_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41662);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN163_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41663);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN164_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41664);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN165_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41665);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN166_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41666);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN167_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41667);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN168_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41668);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN169_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41669);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN170_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41670);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN171_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41671);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN172_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41672);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN173_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41673);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN174_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41674);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN175_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41675);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN176_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41676);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN177_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41677);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN178_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41678);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN179_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41679);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN180_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41680);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN181_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41681);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN182_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41682);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN183_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41683);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN184_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41684);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN185_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41685);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN186_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41686);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN187_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41687);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN188_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41688);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN189_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41689);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN190_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41690);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN191_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41691);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN192_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41692);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN193_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41693);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN194_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41694);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN195_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41695);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN196_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41696);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN197_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41697);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN198_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41698);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN199_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41699);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN200_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41700);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN201_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41701);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN202_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41702);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN203_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41703);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN204_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41704);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN205_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41705);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN206_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41706);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN207_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41707);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN208_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41708);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN209_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41709);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN210_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41710);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN211_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41711);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN212_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41712);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN213_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41713);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN214_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41714);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN215_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41715);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN216_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41716);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN217_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41717);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN218_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41718);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN219_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41719);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN220_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41720);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN221_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41721);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN222_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41722);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN223_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41723);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN224_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41724);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN225_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41725);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN226_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41726);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN227_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41727);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN228_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41728);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN229_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41729);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN230_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41730);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN231_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41731);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN232_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41732);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN233_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41733);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN234_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41734);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN235_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41735);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN236_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41736);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN237_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41737);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN238_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41738);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN239_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41739);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN240_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41740);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN241_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41741);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN242_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41742);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN243_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41743);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN244_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41744);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN245_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41745);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN246_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41746);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN247_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41747);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN248_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41748);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN249_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41749);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN250_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41750);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN251_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41751);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN252_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41752);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN253_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41753);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times.
pub const AIN254_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(41754);
/// Settling time for command-response and AIN-EF readings.
pub const AIN0_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42000);
/// Settling time for command-response and AIN-EF readings.
pub const AIN1_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42002);
/// Settling time for command-response and AIN-EF readings.
pub const AIN2_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42004);
/// Settling time for command-response and AIN-EF readings.
pub const AIN3_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42006);
/// Settling time for command-response and AIN-EF readings.
pub const AIN4_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42008);
/// Settling time for command-response and AIN-EF readings.
pub const AIN5_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42010);
/// Settling time for command-response and AIN-EF readings.
pub const AIN6_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42012);
/// Settling time for command-response and AIN-EF readings.
pub const AIN7_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42014);
/// Settling time for command-response and AIN-EF readings.
pub const AIN8_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42016);
/// Settling time for command-response and AIN-EF readings.
pub const AIN9_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42018);
/// Settling time for command-response and AIN-EF readings.
pub const AIN10_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42020);
/// Settling time for command-response and AIN-EF readings.
pub const AIN11_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42022);
/// Settling time for command-response and AIN-EF readings.
pub const AIN12_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42024);
/// Settling time for command-response and AIN-EF readings.
pub const AIN13_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42026);
/// Settling time for command-response and AIN-EF readings.
pub const AIN14_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42028);
/// Settling time for command-response and AIN-EF readings.
pub const AIN15_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42030);
/// Settling time for command-response and AIN-EF readings.
pub const AIN16_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42032);
/// Settling time for command-response and AIN-EF readings.
pub const AIN17_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42034);
/// Settling time for command-response and AIN-EF readings.
pub const AIN18_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42036);
/// Settling time for command-response and AIN-EF readings.
pub const AIN19_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42038);
/// Settling time for command-response and AIN-EF readings.
pub const AIN20_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42040);
/// Settling time for command-response and AIN-EF readings.
pub const AIN21_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42042);
/// Settling time for command-response and AIN-EF readings.
pub const AIN22_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42044);
/// Settling time for command-response and AIN-EF readings.
pub const AIN23_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42046);
/// Settling time for command-response and AIN-EF readings.
pub const AIN24_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42048);
/// Settling time for command-response and AIN-EF readings.
pub const AIN25_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42050);
/// Settling time for command-response and AIN-EF readings.
pub const AIN26_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42052);
/// Settling time for command-response and AIN-EF readings.
pub const AIN27_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42054);
/// Settling time for command-response and AIN-EF readings.
pub const AIN28_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42056);
/// Settling time for command-response and AIN-EF readings.
pub const AIN29_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42058);
/// Settling time for command-response and AIN-EF readings.
pub const AIN30_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42060);
/// Settling time for command-response and AIN-EF readings.
pub const AIN31_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42062);
/// Settling time for command-response and AIN-EF readings.
pub const AIN32_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42064);
/// Settling time for command-response and AIN-EF readings.
pub const AIN33_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42066);
/// Settling time for command-response and AIN-EF readings.
pub const AIN34_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42068);
/// Settling time for command-response and AIN-EF readings.
pub const AIN35_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42070);
/// Settling time for command-response and AIN-EF readings.
pub const AIN36_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42072);
/// Settling time for command-response and AIN-EF readings.
pub const AIN37_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42074);
/// Settling time for command-response and AIN-EF readings.
pub const AIN38_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42076);
/// Settling time for command-response and AIN-EF readings.
pub const AIN39_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42078);
/// Settling time for command-response and AIN-EF readings.
pub const AIN40_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42080);
/// Settling time for command-response and AIN-EF readings.
pub const AIN41_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42082);
/// Settling time for command-response and AIN-EF readings.
pub const AIN42_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42084);
/// Settling time for command-response and AIN-EF readings.
pub const AIN43_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42086);
/// Settling time for command-response and AIN-EF readings.
pub const AIN44_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42088);
/// Settling time for command-response and AIN-EF readings.
pub const AIN45_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42090);
/// Settling time for command-response and AIN-EF readings.
pub const AIN46_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42092);
/// Settling time for command-response and AIN-EF readings.
pub const AIN47_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42094);
/// Settling time for command-response and AIN-EF readings.
pub const AIN48_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42096);
/// Settling time for command-response and AIN-EF readings.
pub const AIN49_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42098);
/// Settling time for command-response and AIN-EF readings.
pub const AIN50_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42100);
/// Settling time for command-response and AIN-EF readings.
pub const AIN51_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42102);
/// Settling time for command-response and AIN-EF readings.
pub const AIN52_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42104);
/// Settling time for command-response and AIN-EF readings.
pub const AIN53_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42106);
/// Settling time for command-response and AIN-EF readings.
pub const AIN54_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42108);
/// Settling time for command-response and AIN-EF readings.
pub const AIN55_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42110);
/// Settling time for command-response and AIN-EF readings.
pub const AIN56_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42112);
/// Settling time for command-response and AIN-EF readings.
pub const AIN57_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42114);
/// Settling time for command-response and AIN-EF readings.
pub const AIN58_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42116);
/// Settling time for command-response and AIN-EF readings.
pub const AIN59_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42118);
/// Settling time for command-response and AIN-EF readings.
pub const AIN60_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42120);
/// Settling time for command-response and AIN-EF readings.
pub const AIN61_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42122);
/// Settling time for command-response and AIN-EF readings.
pub const AIN62_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42124);
/// Settling time for command-response and AIN-EF readings.
pub const AIN63_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42126);
/// Settling time for command-response and AIN-EF readings.
pub const AIN64_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42128);
/// Settling time for command-response and AIN-EF readings.
pub const AIN65_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42130);
/// Settling time for command-response and AIN-EF readings.
pub const AIN66_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42132);
/// Settling time for command-response and AIN-EF readings.
pub const AIN67_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42134);
/// Settling time for command-response and AIN-EF readings.
pub const AIN68_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42136);
/// Settling time for command-response and AIN-EF readings.
pub const AIN69_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42138);
/// Settling time for command-response and AIN-EF readings.
pub const AIN70_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42140);
/// Settling time for command-response and AIN-EF readings.
pub const AIN71_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42142);
/// Settling time for command-response and AIN-EF readings.
pub const AIN72_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42144);
/// Settling time for command-response and AIN-EF readings.
pub const AIN73_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42146);
/// Settling time for command-response and AIN-EF readings.
pub const AIN74_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42148);
/// Settling time for command-response and AIN-EF readings.
pub const AIN75_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42150);
/// Settling time for command-response and AIN-EF readings.
pub const AIN76_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42152);
/// Settling time for command-response and AIN-EF readings.
pub const AIN77_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42154);
/// Settling time for command-response and AIN-EF readings.
pub const AIN78_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42156);
/// Settling time for command-response and AIN-EF readings.
pub const AIN79_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42158);
/// Settling time for command-response and AIN-EF readings.
pub const AIN80_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42160);
/// Settling time for command-response and AIN-EF readings.
pub const AIN81_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42162);
/// Settling time for command-response and AIN-EF readings.
pub const AIN82_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42164);
/// Settling time for command-response and AIN-EF readings.
pub const AIN83_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42166);
/// Settling time for command-response and AIN-EF readings.
pub const AIN84_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42168);
/// Settling time for command-response and AIN-EF readings.
pub const AIN85_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42170);
/// Settling time for command-response and AIN-EF readings.
pub const AIN86_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42172);
/// Settling time for command-response and AIN-EF readings.
pub const AIN87_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42174);
/// Settling time for command-response and AIN-EF readings.
pub const AIN88_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42176);
/// Settling time for command-response and AIN-EF readings.
pub const AIN89_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42178);
/// Settling time for command-response and AIN-EF readings.
pub const AIN90_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42180);
/// Settling time for command-response and AIN-EF readings.
pub const AIN91_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42182);
/// Settling time for command-response and AIN-EF readings.
pub const AIN92_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42184);
/// Settling time for command-response and AIN-EF readings.
pub const AIN93_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42186);
/// Settling time for command-response and AIN-EF readings.
pub const AIN94_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42188);
/// Settling time for command-response and AIN-EF readings.
pub const AIN95_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42190);
/// Settling time for command-response and AIN-EF readings.
pub const AIN96_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42192);
/// Settling time for command-response and AIN-EF readings.
pub const AIN97_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42194);
/// Settling time for command-response and AIN-EF readings.
pub const AIN98_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42196);
/// Settling time for command-response and AIN-EF readings.
pub const AIN99_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42198);
/// Settling time for command-response and AIN-EF readings.
pub const AIN100_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42200);
/// Settling time for command-response and AIN-EF readings.
pub const AIN101_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42202);
/// Settling time for command-response and AIN-EF readings.
pub const AIN102_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42204);
/// Settling time for command-response and AIN-EF readings.
pub const AIN103_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42206);
/// Settling time for command-response and AIN-EF readings.
pub const AIN104_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42208);
/// Settling time for command-response and AIN-EF readings.
pub const AIN105_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42210);
/// Settling time for command-response and AIN-EF readings.
pub const AIN106_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42212);
/// Settling time for command-response and AIN-EF readings.
pub const AIN107_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42214);
/// Settling time for command-response and AIN-EF readings.
pub const AIN108_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42216);
/// Settling time for command-response and AIN-EF readings.
pub const AIN109_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42218);
/// Settling time for command-response and AIN-EF readings.
pub const AIN110_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42220);
/// Settling time for command-response and AIN-EF readings.
pub const AIN111_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42222);
/// Settling time for command-response and AIN-EF readings.
pub const AIN112_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42224);
/// Settling time for command-response and AIN-EF readings.
pub const AIN113_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42226);
/// Settling time for command-response and AIN-EF readings.
pub const AIN114_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42228);
/// Settling time for command-response and AIN-EF readings.
pub const AIN115_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42230);
/// Settling time for command-response and AIN-EF readings.
pub const AIN116_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42232);
/// Settling time for command-response and AIN-EF readings.
pub const AIN117_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42234);
/// Settling time for command-response and AIN-EF readings.
pub const AIN118_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42236);
/// Settling time for command-response and AIN-EF readings.
pub const AIN119_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42238);
/// Settling time for command-response and AIN-EF readings.
pub const AIN120_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42240);
/// Settling time for command-response and AIN-EF readings.
pub const AIN121_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42242);
/// Settling time for command-response and AIN-EF readings.
pub const AIN122_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42244);
/// Settling time for command-response and AIN-EF readings.
pub const AIN123_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42246);
/// Settling time for command-response and AIN-EF readings.
pub const AIN124_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42248);
/// Settling time for command-response and AIN-EF readings.
pub const AIN125_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42250);
/// Settling time for command-response and AIN-EF readings.
pub const AIN126_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42252);
/// Settling time for command-response and AIN-EF readings.
pub const AIN127_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42254);
/// Settling time for command-response and AIN-EF readings.
pub const AIN128_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42256);
/// Settling time for command-response and AIN-EF readings.
pub const AIN129_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42258);
/// Settling time for command-response and AIN-EF readings.
pub const AIN130_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42260);
/// Settling time for command-response and AIN-EF readings.
pub const AIN131_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42262);
/// Settling time for command-response and AIN-EF readings.
pub const AIN132_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42264);
/// Settling time for command-response and AIN-EF readings.
pub const AIN133_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42266);
/// Settling time for command-response and AIN-EF readings.
pub const AIN134_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42268);
/// Settling time for command-response and AIN-EF readings.
pub const AIN135_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42270);
/// Settling time for command-response and AIN-EF readings.
pub const AIN136_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42272);
/// Settling time for command-response and AIN-EF readings.
pub const AIN137_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42274);
/// Settling time for command-response and AIN-EF readings.
pub const AIN138_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42276);
/// Settling time for command-response and AIN-EF readings.
pub const AIN139_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42278);
/// Settling time for command-response and AIN-EF readings.
pub const AIN140_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42280);
/// Settling time for command-response and AIN-EF readings.
pub const AIN141_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42282);
/// Settling time for command-response and AIN-EF readings.
pub const AIN142_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42284);
/// Settling time for command-response and AIN-EF readings.
pub const AIN143_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42286);
/// Settling time for command-response and AIN-EF readings.
pub const AIN144_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42288);
/// Settling time for command-response and AIN-EF readings.
pub const AIN145_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42290);
/// Settling time for command-response and AIN-EF readings.
pub const AIN146_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42292);
/// Settling time for command-response and AIN-EF readings.
pub const AIN147_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42294);
/// Settling time for command-response and AIN-EF readings.
pub const AIN148_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42296);
/// Settling time for command-response and AIN-EF readings.
pub const AIN149_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42298);
/// Settling time for command-response and AIN-EF readings.
pub const AIN150_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42300);
/// Settling time for command-response and AIN-EF readings.
pub const AIN151_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42302);
/// Settling time for command-response and AIN-EF readings.
pub const AIN152_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42304);
/// Settling time for command-response and AIN-EF readings.
pub const AIN153_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42306);
/// Settling time for command-response and AIN-EF readings.
pub const AIN154_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42308);
/// Settling time for command-response and AIN-EF readings.
pub const AIN155_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42310);
/// Settling time for command-response and AIN-EF readings.
pub const AIN156_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42312);
/// Settling time for command-response and AIN-EF readings.
pub const AIN157_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42314);
/// Settling time for command-response and AIN-EF readings.
pub const AIN158_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42316);
/// Settling time for command-response and AIN-EF readings.
pub const AIN159_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42318);
/// Settling time for command-response and AIN-EF readings.
pub const AIN160_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42320);
/// Settling time for command-response and AIN-EF readings.
pub const AIN161_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42322);
/// Settling time for command-response and AIN-EF readings.
pub const AIN162_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42324);
/// Settling time for command-response and AIN-EF readings.
pub const AIN163_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42326);
/// Settling time for command-response and AIN-EF readings.
pub const AIN164_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42328);
/// Settling time for command-response and AIN-EF readings.
pub const AIN165_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42330);
/// Settling time for command-response and AIN-EF readings.
pub const AIN166_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42332);
/// Settling time for command-response and AIN-EF readings.
pub const AIN167_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42334);
/// Settling time for command-response and AIN-EF readings.
pub const AIN168_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42336);
/// Settling time for command-response and AIN-EF readings.
pub const AIN169_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42338);
/// Settling time for command-response and AIN-EF readings.
pub const AIN170_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42340);
/// Settling time for command-response and AIN-EF readings.
pub const AIN171_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42342);
/// Settling time for command-response and AIN-EF readings.
pub const AIN172_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42344);
/// Settling time for command-response and AIN-EF readings.
pub const AIN173_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42346);
/// Settling time for command-response and AIN-EF readings.
pub const AIN174_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42348);
/// Settling time for command-response and AIN-EF readings.
pub const AIN175_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42350);
/// Settling time for command-response and AIN-EF readings.
pub const AIN176_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42352);
/// Settling time for command-response and AIN-EF readings.
pub const AIN177_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42354);
/// Settling time for command-response and AIN-EF readings.
pub const AIN178_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42356);
/// Settling time for command-response and AIN-EF readings.
pub const AIN179_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42358);
/// Settling time for command-response and AIN-EF readings.
pub const AIN180_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42360);
/// Settling time for command-response and AIN-EF readings.
pub const AIN181_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42362);
/// Settling time for command-response and AIN-EF readings.
pub const AIN182_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42364);
/// Settling time for command-response and AIN-EF readings.
pub const AIN183_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42366);
/// Settling time for command-response and AIN-EF readings.
pub const AIN184_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42368);
/// Settling time for command-response and AIN-EF readings.
pub const AIN185_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42370);
/// Settling time for command-response and AIN-EF readings.
pub const AIN186_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42372);
/// Settling time for command-response and AIN-EF readings.
pub const AIN187_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42374);
/// Settling time for command-response and AIN-EF readings.
pub const AIN188_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42376);
/// Settling time for command-response and AIN-EF readings.
pub const AIN189_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42378);
/// Settling time for command-response and AIN-EF readings.
pub const AIN190_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42380);
/// Settling time for command-response and AIN-EF readings.
pub const AIN191_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42382);
/// Settling time for command-response and AIN-EF readings.
pub const AIN192_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42384);
/// Settling time for command-response and AIN-EF readings.
pub const AIN193_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42386);
/// Settling time for command-response and AIN-EF readings.
pub const AIN194_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42388);
/// Settling time for command-response and AIN-EF readings.
pub const AIN195_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42390);
/// Settling time for command-response and AIN-EF readings.
pub const AIN196_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42392);
/// Settling time for command-response and AIN-EF readings.
pub const AIN197_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42394);
/// Settling time for command-response and AIN-EF readings.
pub const AIN198_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42396);
/// Settling time for command-response and AIN-EF readings.
pub const AIN199_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42398);
/// Settling time for command-response and AIN-EF readings.
pub const AIN200_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42400);
/// Settling time for command-response and AIN-EF readings.
pub const AIN201_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42402);
/// Settling time for command-response and AIN-EF readings.
pub const AIN202_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42404);
/// Settling time for command-response and AIN-EF readings.
pub const AIN203_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42406);
/// Settling time for command-response and AIN-EF readings.
pub const AIN204_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42408);
/// Settling time for command-response and AIN-EF readings.
pub const AIN205_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42410);
/// Settling time for command-response and AIN-EF readings.
pub const AIN206_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42412);
/// Settling time for command-response and AIN-EF readings.
pub const AIN207_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42414);
/// Settling time for command-response and AIN-EF readings.
pub const AIN208_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42416);
/// Settling time for command-response and AIN-EF readings.
pub const AIN209_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42418);
/// Settling time for command-response and AIN-EF readings.
pub const AIN210_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42420);
/// Settling time for command-response and AIN-EF readings.
pub const AIN211_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42422);
/// Settling time for command-response and AIN-EF readings.
pub const AIN212_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42424);
/// Settling time for command-response and AIN-EF readings.
pub const AIN213_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42426);
/// Settling time for command-response and AIN-EF readings.
pub const AIN214_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42428);
/// Settling time for command-response and AIN-EF readings.
pub const AIN215_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42430);
/// Settling time for command-response and AIN-EF readings.
pub const AIN216_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42432);
/// Settling time for command-response and AIN-EF readings.
pub const AIN217_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42434);
/// Settling time for command-response and AIN-EF readings.
pub const AIN218_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42436);
/// Settling time for command-response and AIN-EF readings.
pub const AIN219_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42438);
/// Settling time for command-response and AIN-EF readings.
pub const AIN220_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42440);
/// Settling time for command-response and AIN-EF readings.
pub const AIN221_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42442);
/// Settling time for command-response and AIN-EF readings.
pub const AIN222_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42444);
/// Settling time for command-response and AIN-EF readings.
pub const AIN223_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42446);
/// Settling time for command-response and AIN-EF readings.
pub const AIN224_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42448);
/// Settling time for command-response and AIN-EF readings.
pub const AIN225_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42450);
/// Settling time for command-response and AIN-EF readings.
pub const AIN226_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42452);
/// Settling time for command-response and AIN-EF readings.
pub const AIN227_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42454);
/// Settling time for command-response and AIN-EF readings.
pub const AIN228_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42456);
/// Settling time for command-response and AIN-EF readings.
pub const AIN229_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42458);
/// Settling time for command-response and AIN-EF readings.
pub const AIN230_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42460);
/// Settling time for command-response and AIN-EF readings.
pub const AIN231_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42462);
/// Settling time for command-response and AIN-EF readings.
pub const AIN232_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42464);
/// Settling time for command-response and AIN-EF readings.
pub const AIN233_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42466);
/// Settling time for command-response and AIN-EF readings.
pub const AIN234_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42468);
/// Settling time for command-response and AIN-EF readings.
pub const AIN235_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42470);
/// Settling time for command-response and AIN-EF readings.
pub const AIN236_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42472);
/// Settling time for command-response and AIN-EF readings.
pub const AIN237_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42474);
/// Settling time for command-response and AIN-EF readings.
pub const AIN238_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42476);
/// Settling time for command-response and AIN-EF readings.
pub const AIN239_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42478);
/// Settling time for command-response and AIN-EF readings.
pub const AIN240_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42480);
/// Settling time for command-response and AIN-EF readings.
pub const AIN241_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42482);
/// Settling time for command-response and AIN-EF readings.
pub const AIN242_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42484);
/// Settling time for command-response and AIN-EF readings.
pub const AIN243_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42486);
/// Settling time for command-response and AIN-EF readings.
pub const AIN244_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42488);
/// Settling time for command-response and AIN-EF readings.
pub const AIN245_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42490);
/// Settling time for command-response and AIN-EF readings.
pub const AIN246_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42492);
/// Settling time for command-response and AIN-EF readings.
pub const AIN247_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42494);
/// Settling time for command-response and AIN-EF readings.
pub const AIN248_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42496);
/// Settling time for command-response and AIN-EF readings.
pub const AIN249_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42498);
/// Settling time for command-response and AIN-EF readings.
pub const AIN250_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42500);
/// Settling time for command-response and AIN-EF readings.
pub const AIN251_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42502);
/// Settling time for command-response and AIN-EF readings.
pub const AIN252_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42504);
/// Settling time for command-response and AIN-EF readings.
pub const AIN253_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42506);
/// Settling time for command-response and AIN-EF readings.
pub const AIN254_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(42508);
/// T8 Only. This register is a bitmask representing the enable states of the analog inputs.
pub const AIN_CHANNEL_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(43700);
/// T8 Only. Returns the AIN system's actual resolution index
pub const AIN_ALL_RESOLUTION_INDEX_ACTUAL: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(43702);
/// T8 Only. Writing to this register sets the analog sampling rate. Not all rates are possible. Read AIN_SAMPLING_RATE_ACTUAL_HZ to get the actual rate.
pub const AIN_SAMPLING_RATE_HZ: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(43710);
/// T8 Only. Returns the AIN system's actual sampling rate.
pub const AIN_SAMPLING_RATE_ACTUAL_HZ: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(43712);
/// T8 Only. Returns the status of each AIN channel where 1 is active and 0 deactivated.
pub const AIN_HEALTH: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(43722);
/// The range/span of a all analog inputs. Writing to this register will set the range for all analog inputs. Reading will return a range value if all channels are set to same range. Otherwise, -9999 will be returned.
pub const AIN_ALL_RANGE: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(43900);
/// A write to this global parameter affects all AIN. Writing 1 will set all AINs to differential. Writing 199 will set all AINs to single-ended. A read will return 1 if all AINs are set to differential and 199 if all AINs are set to single-ended. If AIN configurations are not consistent 0xFFFF will be returned.
pub const AIN_ALL_NEGATIVE_CH: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(43902);
/// The resolution index for command-response and AIN-EF readings. A larger resolution index generally results in lower noise and longer sample times. A write to this global parameter affects all AIN.  A read will return the correct setting if all channels are set the same, but otherwise will return 0xFFFF.
pub const AIN_ALL_RESOLUTION_INDEX: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(43903);
/// Settling time for command-response and AIN-EF readings. A write to this global parameter affects all AIN.  A read will return the correct setting if all channels are set the same, but otherwise will return -9999. Max is 50,000 us.
pub const AIN_ALL_SETTLING_US: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(43904);
/// Write 0 to deactivate AIN_EF on all AINs. No other values may be written to this register. Reads will return the AIN_EF index if all 128 AINs are set to the same value. If values are not the same returns 0xFFFF (65535).
pub const AIN_ALL_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(43906);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO0_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44000);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO1_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44002);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO2_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44004);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO3_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44006);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO4_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44008);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO5_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44010);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO6_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44012);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO7_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44014);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO8_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44016);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO9_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44018);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO10_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44020);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO11_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44022);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO12_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44024);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO13_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44026);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO14_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44028);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO15_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44030);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO16_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44032);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO17_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44034);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO18_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44036);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO19_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44038);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO20_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44040);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO21_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44042);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration. Note that DIO-EF reads work when disabled and do not return an error.
pub const DIO22_EF_ENABLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44044);
/// An index to specify the feature you want.
pub const DIO0_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44100);
/// An index to specify the feature you want.
pub const DIO1_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44102);
/// An index to specify the feature you want.
pub const DIO2_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44104);
/// An index to specify the feature you want.
pub const DIO3_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44106);
/// An index to specify the feature you want.
pub const DIO4_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44108);
/// An index to specify the feature you want.
pub const DIO5_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44110);
/// An index to specify the feature you want.
pub const DIO6_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44112);
/// An index to specify the feature you want.
pub const DIO7_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44114);
/// An index to specify the feature you want.
pub const DIO8_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44116);
/// An index to specify the feature you want.
pub const DIO9_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44118);
/// An index to specify the feature you want.
pub const DIO10_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44120);
/// An index to specify the feature you want.
pub const DIO11_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44122);
/// An index to specify the feature you want.
pub const DIO12_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44124);
/// An index to specify the feature you want.
pub const DIO13_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44126);
/// An index to specify the feature you want.
pub const DIO14_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44128);
/// An index to specify the feature you want.
pub const DIO15_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44130);
/// An index to specify the feature you want.
pub const DIO16_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44132);
/// An index to specify the feature you want.
pub const DIO17_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44134);
/// An index to specify the feature you want.
pub const DIO18_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44136);
/// An index to specify the feature you want.
pub const DIO19_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44138);
/// An index to specify the feature you want.
pub const DIO20_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44140);
/// An index to specify the feature you want.
pub const DIO21_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44142);
/// An index to specify the feature you want.
pub const DIO22_EF_INDEX: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44144);
/// Set Clock Source to use for selected feature index.
pub const DIO0_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44200);
/// Set Clock Source to use for selected feature index.
pub const DIO1_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44202);
/// Set Clock Source to use for selected feature index.
pub const DIO2_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44204);
/// Set Clock Source to use for selected feature index.
pub const DIO3_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44206);
/// Set Clock Source to use for selected feature index.
pub const DIO4_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44208);
/// Set Clock Source to use for selected feature index.
pub const DIO5_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44210);
/// Set Clock Source to use for selected feature index.
pub const DIO6_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44212);
/// Set Clock Source to use for selected feature index.
pub const DIO7_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44214);
/// Set Clock Source to use for selected feature index.
pub const DIO8_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44216);
/// Set Clock Source to use for selected feature index.
pub const DIO9_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44218);
/// Set Clock Source to use for selected feature index.
pub const DIO10_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44220);
/// Set Clock Source to use for selected feature index.
pub const DIO11_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44222);
/// Set Clock Source to use for selected feature index.
pub const DIO12_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44224);
/// Set Clock Source to use for selected feature index.
pub const DIO13_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44226);
/// Set Clock Source to use for selected feature index.
pub const DIO14_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44228);
/// Set Clock Source to use for selected feature index.
pub const DIO15_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44230);
/// Set Clock Source to use for selected feature index.
pub const DIO16_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44232);
/// Set Clock Source to use for selected feature index.
pub const DIO17_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44234);
/// Set Clock Source to use for selected feature index.
pub const DIO18_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44236);
/// Set Clock Source to use for selected feature index.
pub const DIO19_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44238);
/// Set Clock Source to use for selected feature index.
pub const DIO20_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44240);
/// Set Clock Source to use for selected feature index.
pub const DIO21_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44242);
/// Set Clock Source to use for selected feature index.
pub const DIO22_EF_CLOCK_SOURCE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44244);
/// Function dependent on selected feature index.
pub const DIO0_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44300);
/// Function dependent on selected feature index.
pub const DIO1_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44302);
/// Function dependent on selected feature index.
pub const DIO2_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44304);
/// Function dependent on selected feature index.
pub const DIO3_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44306);
/// Function dependent on selected feature index.
pub const DIO4_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44308);
/// Function dependent on selected feature index.
pub const DIO5_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44310);
/// Function dependent on selected feature index.
pub const DIO6_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44312);
/// Function dependent on selected feature index.
pub const DIO7_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44314);
/// Function dependent on selected feature index.
pub const DIO8_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44316);
/// Function dependent on selected feature index.
pub const DIO9_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44318);
/// Function dependent on selected feature index.
pub const DIO10_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44320);
/// Function dependent on selected feature index.
pub const DIO11_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44322);
/// Function dependent on selected feature index.
pub const DIO12_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44324);
/// Function dependent on selected feature index.
pub const DIO13_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44326);
/// Function dependent on selected feature index.
pub const DIO14_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44328);
/// Function dependent on selected feature index.
pub const DIO15_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44330);
/// Function dependent on selected feature index.
pub const DIO16_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44332);
/// Function dependent on selected feature index.
pub const DIO17_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44334);
/// Function dependent on selected feature index.
pub const DIO18_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44336);
/// Function dependent on selected feature index.
pub const DIO19_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44338);
/// Function dependent on selected feature index.
pub const DIO20_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44340);
/// Function dependent on selected feature index.
pub const DIO21_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44342);
/// Function dependent on selected feature index.
pub const DIO22_EF_CONFIG_A: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44344);
/// Function dependent on selected feature index.
pub const DIO0_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44400);
/// Function dependent on selected feature index.
pub const DIO1_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44402);
/// Function dependent on selected feature index.
pub const DIO2_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44404);
/// Function dependent on selected feature index.
pub const DIO3_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44406);
/// Function dependent on selected feature index.
pub const DIO4_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44408);
/// Function dependent on selected feature index.
pub const DIO5_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44410);
/// Function dependent on selected feature index.
pub const DIO6_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44412);
/// Function dependent on selected feature index.
pub const DIO7_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44414);
/// Function dependent on selected feature index.
pub const DIO8_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44416);
/// Function dependent on selected feature index.
pub const DIO9_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44418);
/// Function dependent on selected feature index.
pub const DIO10_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44420);
/// Function dependent on selected feature index.
pub const DIO11_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44422);
/// Function dependent on selected feature index.
pub const DIO12_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44424);
/// Function dependent on selected feature index.
pub const DIO13_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44426);
/// Function dependent on selected feature index.
pub const DIO14_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44428);
/// Function dependent on selected feature index.
pub const DIO15_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44430);
/// Function dependent on selected feature index.
pub const DIO16_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44432);
/// Function dependent on selected feature index.
pub const DIO17_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44434);
/// Function dependent on selected feature index.
pub const DIO18_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44436);
/// Function dependent on selected feature index.
pub const DIO19_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44438);
/// Function dependent on selected feature index.
pub const DIO20_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44440);
/// Function dependent on selected feature index.
pub const DIO21_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44442);
/// Function dependent on selected feature index.
pub const DIO22_EF_CONFIG_B: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44444);
/// Function dependent on selected feature index.
pub const DIO0_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44500);
/// Function dependent on selected feature index.
pub const DIO1_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44502);
/// Function dependent on selected feature index.
pub const DIO2_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44504);
/// Function dependent on selected feature index.
pub const DIO3_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44506);
/// Function dependent on selected feature index.
pub const DIO4_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44508);
/// Function dependent on selected feature index.
pub const DIO5_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44510);
/// Function dependent on selected feature index.
pub const DIO6_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44512);
/// Function dependent on selected feature index.
pub const DIO7_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44514);
/// Function dependent on selected feature index.
pub const DIO8_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44516);
/// Function dependent on selected feature index.
pub const DIO9_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44518);
/// Function dependent on selected feature index.
pub const DIO10_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44520);
/// Function dependent on selected feature index.
pub const DIO11_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44522);
/// Function dependent on selected feature index.
pub const DIO12_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44524);
/// Function dependent on selected feature index.
pub const DIO13_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44526);
/// Function dependent on selected feature index.
pub const DIO14_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44528);
/// Function dependent on selected feature index.
pub const DIO15_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44530);
/// Function dependent on selected feature index.
pub const DIO16_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44532);
/// Function dependent on selected feature index.
pub const DIO17_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44534);
/// Function dependent on selected feature index.
pub const DIO18_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44536);
/// Function dependent on selected feature index.
pub const DIO19_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44538);
/// Function dependent on selected feature index.
pub const DIO20_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44540);
/// Function dependent on selected feature index.
pub const DIO21_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44542);
/// Function dependent on selected feature index.
pub const DIO22_EF_CONFIG_C: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44544);
/// Function dependent on selected feature index.
pub const DIO0_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44600);
/// Function dependent on selected feature index.
pub const DIO1_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44602);
/// Function dependent on selected feature index.
pub const DIO2_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44604);
/// Function dependent on selected feature index.
pub const DIO3_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44606);
/// Function dependent on selected feature index.
pub const DIO4_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44608);
/// Function dependent on selected feature index.
pub const DIO5_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44610);
/// Function dependent on selected feature index.
pub const DIO6_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44612);
/// Function dependent on selected feature index.
pub const DIO7_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44614);
/// Function dependent on selected feature index.
pub const DIO8_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44616);
/// Function dependent on selected feature index.
pub const DIO9_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44618);
/// Function dependent on selected feature index.
pub const DIO10_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44620);
/// Function dependent on selected feature index.
pub const DIO11_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44622);
/// Function dependent on selected feature index.
pub const DIO12_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44624);
/// Function dependent on selected feature index.
pub const DIO13_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44626);
/// Function dependent on selected feature index.
pub const DIO14_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44628);
/// Function dependent on selected feature index.
pub const DIO15_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44630);
/// Function dependent on selected feature index.
pub const DIO16_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44632);
/// Function dependent on selected feature index.
pub const DIO17_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44634);
/// Function dependent on selected feature index.
pub const DIO18_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44636);
/// Function dependent on selected feature index.
pub const DIO19_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44638);
/// Function dependent on selected feature index.
pub const DIO20_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44640);
/// Function dependent on selected feature index.
pub const DIO21_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44642);
/// Function dependent on selected feature index.
pub const DIO22_EF_CONFIG_D: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44644);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration.
pub const DIO_EF_CLOCK0_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(44900);
/// Divides the core clock. Valid options: 1,2,4,8,16,32,64,256.
pub const DIO_EF_CLOCK0_DIVISOR: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(44901);
/// Bitmask: bit0: 1 = use external clock. All other bits reserved.
pub const DIO_EF_CLOCK0_OPTIONS: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44902);
/// The clock count will increment continuously and then start over at zero as it reaches the roll value. DIO_EF_CLOCK0 is a 32-bit clock, valid values are 0 to (2^32 - 1). 0 results in the max roll value (2^32).
pub const DIO_EF_CLOCK0_ROLL_VALUE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44904);
/// Current tick count of this clock.  Will read between 0 and ROLL_VALUE-1.
pub const DIO_EF_CLOCK0_COUNT: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(44908);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration.
pub const DIO_EF_CLOCK1_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(44910);
/// Divides the core clock. Valid options: 1,2,4,8,16,32,64,256.
pub const DIO_EF_CLOCK1_DIVISOR: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(44911);
/// Bitmask: bit0: 1 = use external clock. All other bits reserved.
pub const DIO_EF_CLOCK1_OPTIONS: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44912);
/// The clock will count to this value and then start over at zero. The clock pulses counted are those after the divisor. 0 results in the max roll value possible. This is a 32-bit value (0-4294967295) if using a 32-bit clock, and a 16-bit value (0-65535) if using a 16-bit clock.
pub const DIO_EF_CLOCK1_ROLL_VALUE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44914);
/// Current tick count of this clock.  Will read between 0 and ROLL_VALUE-1.
pub const DIO_EF_CLOCK1_COUNT: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(44918);
/// 1 = enabled. 0 = disabled. Must be disabled during configuration.
pub const DIO_EF_CLOCK2_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(44920);
/// Divides the core clock. Valid options: 1,2,4,8,16,32,64,256.
pub const DIO_EF_CLOCK2_DIVISOR: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(44921);
/// Bitmask: bit0: 1 = use external clock. All other bits reserved.
pub const DIO_EF_CLOCK2_OPTIONS: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44922);
/// The clock will count to this value and then start over at zero. The clock pulses counted are those after the divisor. 0 results in the max roll value possible. This is a 32-bit value (0-4294967295) if using a 32-bit clock, and a 16-bit value (0-65535) if using a 16-bit clock.
pub const DIO_EF_CLOCK2_ROLL_VALUE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(44924);
/// Current tick count of this clock.  Will read between 0 and ROLL_VALUE-1.
pub const DIO_EF_CLOCK2_COUNT: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(44928);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO0_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45000);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO1_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45002);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO2_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45004);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO3_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45006);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO4_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45008);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO5_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45010);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO6_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45012);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO7_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45014);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO8_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45016);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO9_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45018);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO10_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45020);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO11_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45022);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO12_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45024);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO13_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45026);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO14_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45028);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO15_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45030);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO16_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45032);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO17_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45034);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO18_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45036);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO19_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45038);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO20_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45040);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO21_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45042);
/// Deprecated. Automatically configures the associated DIO_EF for feature index 11. If already configured for index 11, reads the result and resets the DIO_EF.
pub const DIO22_EF_EASY_FREQUENCY_IN: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(45044);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM0_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46000);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM1_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46002);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM2_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46004);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM3_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46006);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM4_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46008);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM5_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46010);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM6_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46012);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM7_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46014);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM8_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46016);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM9_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46018);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM10_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46020);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM11_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46022);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM12_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46024);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM13_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46026);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM14_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46028);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM15_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46030);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM16_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46032);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM17_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46034);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM18_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46036);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM19_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46038);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM20_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46040);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM21_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46042);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM22_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46044);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM23_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46046);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM24_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46048);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM25_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46050);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM26_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46052);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM27_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46054);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM28_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46056);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM29_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46058);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM30_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46060);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM31_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46062);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM32_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46064);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM33_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46066);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM34_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46068);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM35_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46070);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM36_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46072);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM37_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46074);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM38_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46076);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM39_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(46078);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM0_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46080);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM1_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46082);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM2_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46084);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM3_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46086);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM4_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46088);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM5_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46090);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM6_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46092);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM7_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46094);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM8_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46096);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM9_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(46098);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM0_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46100);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM1_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46102);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM2_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46104);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM3_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46106);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM4_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46108);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM5_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46110);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM6_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46112);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM7_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46114);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM8_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46116);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM9_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46118);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM10_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46120);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM11_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46122);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM12_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46124);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM13_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46126);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM14_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46128);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM15_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46130);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM16_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46132);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM17_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46134);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM18_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46136);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM19_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46138);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM20_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46140);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM21_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46142);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM22_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46144);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM23_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46146);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM24_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46148);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM25_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46150);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM26_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46152);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM27_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46154);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM28_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46156);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM29_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46158);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM30_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46160);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM31_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46162);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM32_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46164);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM33_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46166);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM34_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46168);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM35_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46170);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM36_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46172);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM37_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46174);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM38_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46176);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM39_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(46178);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM0_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46180);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM1_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46181);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM2_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46182);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM3_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46183);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM4_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46184);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM5_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46185);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM6_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46186);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM7_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46187);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM8_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46188);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM9_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46189);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM10_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46190);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM11_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46191);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM12_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46192);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM13_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46193);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM14_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46194);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM15_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46195);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM16_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46196);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM17_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46197);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM18_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46198);
/// Generic RAM registers. Useful for passing data between a host computer and a Lua script. Will not return an error if alternate data types are used.
pub const USER_RAM19_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(46199);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO0_DATA_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(47000);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO1_DATA_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(47001);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO2_DATA_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(47002);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO3_DATA_U16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(47003);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO0_DATA_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(47010);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO1_DATA_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(47012);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO2_DATA_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(47014);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO3_DATA_U32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(47016);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO0_DATA_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(47020);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO1_DATA_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(47022);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO2_DATA_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(47024);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO3_DATA_I32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(47026);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO0_DATA_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(47030);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO1_DATA_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(47032);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO2_DATA_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(47034);
/// Generic FIFO buffer.  Useful for passing ORDERED or SEQUENTIAL data between various endpoints, such as between a host and a Lua script. Use up to 4 FIFO buffers simultaneously->1 of each data type, all 4 different data types, or a mixture. e.g. FIFO0_DATA_U16 points to the same memory as other FIFO0 registers, such that there are a total of 4 memory blocks: FIFO0, FIFO1, FIFO2 and FIFO3.  It is possible to write into a FIFO buffer using a different datatype than is being used to read out of it. This register is a buffer. Underrun behavior - throws an error.
pub const USER_RAM_FIFO3_DATA_F32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(47036);
/// Allocate memory for a FIFO buffer. Number of bytes should be sufficient to store users max transfer array size.  Note that FLOAT32, INT32, and UINT32 require 4 bytes per value, and UINT16 require 2 bytes per value. Maximum size is limited by available memory. Care should be taken to conserve enough memory for other operations such as AIN_EF, Lua, Stream etc.
pub const USER_RAM_FIFO0_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(47900);
/// Allocate memory for a FIFO buffer. Number of bytes should be sufficient to store users max transfer array size.  Note that FLOAT32, INT32, and UINT32 require 4 bytes per value, and UINT16 require 2 bytes per value. Maximum size is limited by available memory. Care should be taken to conserve enough memory for other operations such as AIN_EF, Lua, Stream etc.
pub const USER_RAM_FIFO1_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(47902);
/// Allocate memory for a FIFO buffer. Number of bytes should be sufficient to store users max transfer array size.  Note that FLOAT32, INT32, and UINT32 require 4 bytes per value, and UINT16 require 2 bytes per value. Maximum size is limited by available memory. Care should be taken to conserve enough memory for other operations such as AIN_EF, Lua, Stream etc.
pub const USER_RAM_FIFO2_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(47904);
/// Allocate memory for a FIFO buffer. Number of bytes should be sufficient to store users max transfer array size.  Note that FLOAT32, INT32, and UINT32 require 4 bytes per value, and UINT16 require 2 bytes per value. Maximum size is limited by available memory. Care should be taken to conserve enough memory for other operations such as AIN_EF, Lua, Stream etc.
pub const USER_RAM_FIFO3_ALLOCATE_NUM_BYTES: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(47906);
/// Poll this register to see when new data is available/ready. Each read of the FIFO buffer decreases this value, and each write to the FIFO buffer increases this value. At any point in time, the following equation holds: Nbytes = Nwritten - Nread.
pub const USER_RAM_FIFO0_NUM_BYTES_IN_FIFO: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(47910);
/// Poll this register to see when new data is available/ready. Each read of the FIFO buffer decreases this value, and each write to the FIFO buffer increases this value. At any point in time, the following equation holds: Nbytes = Nwritten - Nread.
pub const USER_RAM_FIFO1_NUM_BYTES_IN_FIFO: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(47912);
/// Poll this register to see when new data is available/ready. Each read of the FIFO buffer decreases this value, and each write to the FIFO buffer increases this value. At any point in time, the following equation holds: Nbytes = Nwritten - Nread.
pub const USER_RAM_FIFO2_NUM_BYTES_IN_FIFO: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(47914);
/// Poll this register to see when new data is available/ready. Each read of the FIFO buffer decreases this value, and each write to the FIFO buffer increases this value. At any point in time, the following equation holds: Nbytes = Nwritten - Nread.
pub const USER_RAM_FIFO3_NUM_BYTES_IN_FIFO: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(47916);
/// Write any value to this register to efficiently empty, flush, or otherwise clear data from the FIFO.
pub const USER_RAM_FIFO0_EMPTY: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(47930);
/// Write any value to this register to efficiently empty, flush, or otherwise clear data from the FIFO.
pub const USER_RAM_FIFO1_EMPTY: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(47932);
/// Write any value to this register to efficiently empty, flush, or otherwise clear data from the FIFO.
pub const USER_RAM_FIFO2_EMPTY: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(47934);
/// Write any value to this register to efficiently empty, flush, or otherwise clear data from the FIFO.
pub const USER_RAM_FIFO3_EMPTY: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(47936);
/// The current power management state.
pub const POWER_MODE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48000);
/// The core processor speed. 0=80MHz, 1=20MHz, 2=2MHz, 3=250kHz
pub const POWER_CORE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48001);
/// The current ON/OFF state of the USB module.
pub const POWER_USB: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48002);
/// The current ON/OFF state of the Ethernet module.
pub const POWER_ETHERNET: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48003);
/// The current ON/OFF state of the WiFi module.
pub const POWER_WIFI: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48004);
/// The current ON/OFF state of the analog input module. For T4 this register always returns 1. For the T8, returns 1 if any AIN channels are enabled. To control the T8's analog power configuration use POWER_AIN_CHANNEL_ENABLE.
pub const POWER_AIN: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48005);
/// Sets the LED operation: 0 = Off. Useful for lower power applications. 1 = normal. 2 = Lower power, LEDs will still blink but will normally be off. 3 = Reserved. 4 = Manual, in this mode the LEDs can be user controlled.
pub const POWER_LED: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48006);
/// Sets or reads the ON/OFF state of individual AIN channels. This register is a bitmask where each bit represents a channel. A 1 = on, 0 = off. Only available certain devices.
pub const POWER_AIN_CHANNEL_ENABLE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48007);
/// The power management state after a power-cycle to the device.
pub const POWER_MODE_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48050);
/// Controls the core processor speed after a power-cycle to the device. 0=80MHz, 1=20MHz, 2=2MHz, 3=250kHz
pub const POWER_CORE_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48051);
/// The current ON/OFF state of the USB module after a power-cycle to the device.
pub const POWER_USB_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48052);
/// The ON/OFF state of the Ethernet module after a power-cycle to the device. Provided to optionally reduce power consumption.
pub const POWER_ETHERNET_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48053);
/// The ON/OFF state of the WiFi module after a power-cycle to the device. Provided to optionally reduce power consumption.
pub const POWER_WIFI_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48054);
/// The ON/OFF state of the analog input module after a power-cycle to the device. Provided to optionally reduce power consumption.
pub const POWER_AIN_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48055);
/// The ON/OFF state of the LEDs after a power-cycle to the device.
pub const POWER_LED_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(48056);
/// Sets and reads the startup setting for POWER_AIN_CHANNEL_ENABLE
pub const POWER_AIN_CHANNEL_ENABLE_DEFAULT: LabjackTag<u16, CanRead, CanWrite> =
    LabjackTag::new(48057);
/// Returns 1 if default settings are the same as factory settings.
pub const IO_CONFIG_CHECK_FOR_FACTORY: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(49000);
/// Write a 1 to cause new default (reboot/power-up) values to be saved to flash.  Current values are retrieved and saved as the new defaults. Systems affected: AIN, DIO, DAC, AIN_EF, DIO_EF.
pub const IO_CONFIG_SET_DEFAULT_TO_CURRENT: LabjackTag<u32, CannotRead, CanWrite> =
    LabjackTag::new(49002);
/// Write a 1 to cause new default (reboot/power-up) values to be saved to flash.  Factory values are retrieved and saved as the new defaults. Systems affected: AIN, DIO, DAC, AIN_EF, DIO_EF.
pub const IO_CONFIG_SET_DEFAULT_TO_FACTORY: LabjackTag<u32, CannotRead, CanWrite> =
    LabjackTag::new(49004);
/// The address in the factory configuration data which reads will start from.
pub const IO_CONFIG_FACTORY_PREAD: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49006);
/// Data read from factory configuration
pub const IO_CONFIG_FACTORY_READ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49008);
/// The address in the configuration data which reads will start from.
pub const IO_CONFIG_DEFAULT_PREAD: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49010);
/// Data read from the startup configuration
pub const IO_CONFIG_DEFAULT_READ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49012);
/// The address in the current configuration data which reads will start from.
pub const IO_CONFIG_CURRENT_PREAD: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49014);
/// Data read from the current configuration
pub const IO_CONFIG_CURRENT_READ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49016);
/// Returns 1 if the current configuration matches the default configuration.
pub const IO_CONFIG_CHECK_FOR_DEFAULT: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(49018);
/// Collects the current IO configuration and calculates of a CRC32.
pub const IO_CONFIG_CURRENT_CRC32: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49020);
/// Writing 0x5317052E to this register will trigger a system cleanse.
pub const CLEANSE: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(49090);
/// Read the current IP address of wired Ethernet.
pub const ETHERNET_IP: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49100);
/// Read the current subnet of wired Ethernet.
pub const ETHERNET_SUBNET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49102);
/// Read the current gateway of wired Ethernet.
pub const ETHERNET_GATEWAY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49104);
/// Read the current DNS of wired Ethernet.
pub const ETHERNET_DNS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49106);
/// Read the current Alt DNS of wired Ethernet.
pub const ETHERNET_ALTDNS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49108);
/// Read the current Enabled/Disabled state of Ethernet DHCP.
pub const ETHERNET_DHCP_ENABLE: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(49110);
/// Restricts which Modbus operations are allowed over UDP. When set to 1, only the registers needed for discovering units can be read and any other operation will throw an error.
pub const ETHERNET_UDP_DISCOVERY_ONLY: LabjackTag<u16, CanRead, CannotWrite> =
    LabjackTag::new(49115);
/// The IP address of wired Ethernet after a power-cycle to the device.
pub const ETHERNET_IP_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49150);
/// The subnet of wired Ethernet after a power-cycle to the device.
pub const ETHERNET_SUBNET_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49152);
/// The gateway of wired Ethernet after a power-cycle to the device.
pub const ETHERNET_GATEWAY_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49154);
/// The DNS of wired Ethernet after a power-cycle to the device.
pub const ETHERNET_DNS_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49156);
/// The Alt DNS of wired Ethernet after a power-cycle to the device.
pub const ETHERNET_ALTDNS_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49158);
/// The Enabled/Disabled state of Ethernet DHCP after a power-cycle to the device.
pub const ETHERNET_DHCP_ENABLE_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(49160);
/// The Enabled/Disabled state of ETHERNET_UDP_DISCOVERY_ONLY after a power-cycle to the device.
pub const ETHERNET_UDP_DISCOVERY_ONLY_DEFAULT: LabjackTag<u16, CanRead, CannotWrite> =
    LabjackTag::new(49165);
/// Writing 1 to this register power-cycles Ethernet.  It tells the device to waits 1s before turning off Ethernet and then 500ms before turning it back on.
pub const ETHERNET_APPLY_SETTINGS: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(49190);
/// Read the current IP address of WiFi.
pub const WIFI_IP: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49200);
/// Read the current subnet of WiFi.
pub const WIFI_SUBNET: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49202);
/// Read the current gateway of WiFi.
pub const WIFI_GATEWAY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49204);
/// Read the current Enabled/Disabled state of WiFi DHCP.
pub const WIFI_DHCP_ENABLE: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(49210);
/// Restricts which Modbus operations are allowed over UDP. When set to 1, only the registers needed for discovering units can be read and any other operation will throw an error.
pub const WIFI_UDP_DISCOVERY_ONLY: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(49215);
/// The new IP address of WiFi. Use WIFI_APPLY_SETTINGS.
pub const WIFI_IP_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49250);
/// The new subnet of WiFi.  Use WIFI_APPLY_SETTINGS.
pub const WIFI_SUBNET_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49252);
/// The new gateway of WiFi. Use WIFI_APPLY_SETTINGS.
pub const WIFI_GATEWAY_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49254);
/// The new Enabled/Disabled state of WiFi DHCP. Use WIFI_APPLY_SETTINGS
pub const WIFI_DHCP_ENABLE_DEFAULT: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(49260);
/// The Enabled/Disabled state of WIFI_UDP_DISCOVERY_ONLY after a power-cycle to the device.
pub const WIFI_UDP_DISCOVERY_ONLY_DEFAULT: LabjackTag<u16, CanRead, CannotWrite> =
    LabjackTag::new(49265);
/// Read the current SSID (network name) of WiFi
pub const WIFI_SSID: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(49300);
/// The new SSID (network name) of WiFi. Use WIFI_APPLY_SETTINGS.
pub const WIFI_SSID_DEFAULT: LabjackTag<Bytes, CanRead, CanWrite> = LabjackTag::new(49325);
/// Write the password for the WiFi network, then use WIFI_APPLY_SETTINGS.
pub const WIFI_PASSWORD_DEFAULT: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(49350);
/// Apply all new WiFi settings: IP, Subnet, Gateway, DHCP, SSID, Password. 1=Apply
pub const WIFI_APPLY_SETTINGS: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(49400);
/// Start an update by using USB or Ethernet to write the desired version to update to.
pub const WIFI_FIRMWARE_UPDATE_TO_VERSIONX: LabjackTag<f32, CannotRead, CanWrite> =
    LabjackTag::new(49402);
/// Status Codes: ASSOCIATED = 2900, ASSOCIATING = 2901, ASSOCIATION_FAILED = 2902, UNPOWERED = 2903, BOOTING = 2904, START_FAILED = 2905, APPLYING_SETTINGS = 2906, DHCP_STARTED = 2907, OTHER = 2909
pub const WIFI_STATUS: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(49450);
/// WiFi RSSI (signal strength).  Typical values are -40 for very good, and -75 for very weak.  The T7 microcontroller only gets a new RSSI value from the WiFi module when WiFi communication occurs.
pub const WIFI_RSSI: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(49452);
/// CONFIGURING = 2920, IN_PROGRESS = 2921, REBOOTING = 2923, UPDATE_SUCCESS = 2924, UPDATE_FAILED = 2925
pub const WIFI_FIRMWARE_UPDATE_STATUS: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(49454);
/// Sets the SNTP retry time in seconds. A value of zero will disable SNTP(0=default). Values must be 10 or greater.
pub const SNTP_UPDATE_INTERVAL: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49702);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN0_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50000);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN1_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50002);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN2_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50004);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN3_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50006);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN4_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50008);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN5_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50010);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN6_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50012);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN7_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50014);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN8_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50016);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN9_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50018);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN10_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50020);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN11_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50022);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN12_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50024);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN13_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50026);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN14_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50028);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN15_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50030);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN16_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50032);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN17_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50034);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN18_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50036);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN19_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50038);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN20_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50040);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN21_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50042);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN22_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50044);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN23_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50046);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN24_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50048);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN25_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50050);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN26_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50052);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN27_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50054);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN28_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50056);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN29_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50058);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN30_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50060);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN31_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50062);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN32_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50064);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN33_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50066);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN34_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50068);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN35_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50070);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN36_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50072);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN37_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50074);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN38_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50076);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN39_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50078);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN40_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50080);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN41_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50082);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN42_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50084);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN43_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50086);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN44_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50088);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN45_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50090);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN46_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50092);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN47_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50094);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN48_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50096);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN49_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50098);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN50_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50100);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN51_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50102);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN52_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50104);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN53_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50106);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN54_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50108);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN55_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50110);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN56_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50112);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN57_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50114);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN58_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50116);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN59_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50118);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN60_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50120);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN61_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50122);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN62_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50124);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN63_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50126);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN64_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50128);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN65_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50130);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN66_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50132);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN67_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50134);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN68_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50136);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN69_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50138);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN70_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50140);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN71_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50142);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN72_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50144);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN73_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50146);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN74_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50148);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN75_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50150);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN76_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50152);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN77_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50154);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN78_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50156);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN79_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50158);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN80_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50160);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN81_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50162);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN82_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50164);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN83_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50166);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN84_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50168);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN85_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50170);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN86_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50172);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN87_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50174);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN88_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50176);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN89_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50178);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN90_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50180);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN91_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50182);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN92_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50184);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN93_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50186);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN94_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50188);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN95_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50190);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN96_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50192);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN97_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50194);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN98_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50196);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN99_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50198);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN100_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50200);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN101_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50202);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN102_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50204);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN103_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50206);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN104_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50208);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN105_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50210);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN106_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50212);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN107_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50214);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN108_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50216);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN109_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50218);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN110_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50220);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN111_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50222);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN112_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50224);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN113_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50226);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN114_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50228);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN115_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50230);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN116_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50232);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN117_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50234);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN118_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50236);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN119_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50238);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN120_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50240);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN121_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50242);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN122_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50244);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN123_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50246);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN124_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50248);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN125_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50250);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN126_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50252);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN127_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50254);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN128_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50256);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN129_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50258);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN130_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50260);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN131_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50262);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN132_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50264);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN133_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50266);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN134_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50268);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN135_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50270);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN136_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50272);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN137_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50274);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN138_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50276);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN139_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50278);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN140_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50280);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN141_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50282);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN142_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50284);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN143_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50286);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN144_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50288);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN145_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50290);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN146_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50292);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN147_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50294);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN148_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50296);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN149_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50298);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN150_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50300);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN151_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50302);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN152_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50304);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN153_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50306);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN154_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50308);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN155_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50310);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN156_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50312);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN157_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50314);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN158_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50316);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN159_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50318);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN160_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50320);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN161_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50322);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN162_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50324);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN163_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50326);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN164_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50328);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN165_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50330);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN166_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50332);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN167_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50334);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN168_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50336);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN169_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50338);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN170_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50340);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN171_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50342);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN172_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50344);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN173_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50346);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN174_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50348);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN175_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50350);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN176_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50352);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN177_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50354);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN178_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50356);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN179_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50358);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN180_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50360);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN181_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50362);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN182_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50364);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN183_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50366);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN184_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50368);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN185_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50370);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN186_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50372);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN187_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50374);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN188_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50376);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN189_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50378);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN190_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50380);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN191_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50382);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN192_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50384);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN193_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50386);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN194_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50388);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN195_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50390);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN196_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50392);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN197_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50394);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN198_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50396);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN199_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50398);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN200_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50400);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN201_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50402);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN202_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50404);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN203_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50406);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN204_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50408);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN205_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50410);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN206_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50412);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN207_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50414);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN208_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50416);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN209_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50418);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN210_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50420);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN211_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50422);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN212_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50424);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN213_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50426);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN214_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50428);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN215_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50430);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN216_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50432);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN217_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50434);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN218_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50436);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN219_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50438);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN220_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50440);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN221_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50442);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN222_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50444);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN223_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50446);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN224_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50448);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN225_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50450);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN226_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50452);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN227_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50454);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN228_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50456);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN229_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50458);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN230_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50460);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN231_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50462);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN232_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50464);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN233_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50466);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN234_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50468);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN235_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50470);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN236_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50472);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN237_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50474);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN238_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50476);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN239_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50478);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN240_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50480);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN241_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50482);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN242_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50484);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN243_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50486);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN244_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50488);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN245_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50490);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN246_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50492);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN247_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50494);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN248_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50496);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN249_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50498);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN250_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50500);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN251_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50502);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN252_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50504);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN253_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50506);
/// Returns the 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN254_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50508);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE0_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50600);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE1_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50602);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE2_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50604);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE3_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50606);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE4_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50608);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE5_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50610);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE6_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50612);
/// T8 Only. Returns the 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE7_BINARY: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50614);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN0_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50650);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN1_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50652);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN2_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50654);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN3_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50656);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN4_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50658);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN5_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50660);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN6_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50662);
/// T8 Only. Returns the saved 24-bit binary representation of the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const AIN7_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(50664);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE0_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50700);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE1_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50702);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE2_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50704);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE3_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50706);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE4_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50708);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE5_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50710);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE6_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50712);
/// T8 Only. Returns the saved 24-bit binary representation of the voltage output of the temperature sensor for the specified analog input. If you prefer 16-bit representation, simply divide this by 256. This is for command-response only.  Stream always returns binary and LJM applies cal constants, so the LJM config flag LJM_STREAM_AIN_BINARY is used to get binary values.
pub const TEMPERATURE7_BINARY_CAPTURE: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(50714);
/// Writes binary values to the DACs. Binary values are 16-bit. If the DAC's resolution is less than 16 then the lower bits are ignored. 0 = lowest output, 65535 = highest output.
pub const DAC0_BINARY: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(51000);
/// Writes binary values to the DACs. Binary values are 16-bit. If the DAC's resolution is less than 16 then the lower bits are ignored. 0 = lowest output, 65535 = highest output.
pub const DAC1_BINARY: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(51002);
/// Returns the last LabJack error code seen on the device.
pub const LAST_ERR_DETAIL: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(55000);
/// Returns the last Modbus TCP error code seen on the device.
pub const LAST_MB_ERR: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(55001);
/// Returns the frame number that caused the last error.
pub const LAST_ERR_FRAME: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(55002);
/// Returns the transaction ID of the Modbus TCP packet that caused the last error.
pub const LAST_ERR_TRANSACTION_ID: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(55003);
/// A read of this test register should always return 0x00112233 or d1122867. If your software has the word swap quirk, you will incorrectly read 0x22330011 or 573767697.  If your software has the address-1 quirk, a UINT16 (1-register) read from 55101 will incorrectly return 0x0011 (should read 0x2233).  This register is unlike others, in that it allows you can read a single word from 55100 or 55101, or of course 2 words from 55100.
pub const TEST: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(55100);
/// Write a value and read back to test UINT16 operation. Default is 0x0011 or d17.
pub const TEST_UINT16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(55110);
/// Write a value and read back to test UINT32 operation. Default is 0x00112233 or d1122867. If your software has the word swap quirk, the default will incorrectly read 0x22330011 or 573767697.
pub const TEST_UINT32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(55120);
/// Write a value and read back to test INT32 operation. Default is 0x8899AABB or d-2003195205. If your software has the word swap quirk, the default will incorrectly read 0xAABB8899 or -1430550375.
pub const TEST_INT32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(55122);
/// Write a value and read back to test FLOAT32 operation. Default is 0xC61C3C00 or -9999.0. If your software has the word swap quirk, the default will incorrectly read 0x3C00C61C or 0.00786.
pub const TEST_FLOAT32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(55124);
/// Returns the serial number of an LJTick-DAC, and forces a re-read of the calibration constants. Which LJTDAC is determined by the last write to TDAC# ... whether it was successful or not.
pub const TDAC_SERIAL_NUMBER: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(55200);
/// Sets the I2C clock speed that will be used when communicating with the TDAC. Default value is 65516. See I2C_SPEED_THROTTLE for more detail.
pub const TDAC_SPEED_THROTTLE: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(55202);
/// The numeric identifier of the device.  Such as 7 for a T7 / T7-Pro.
pub const PRODUCT_ID: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60000);
/// The hardware version of the device.
pub const HARDWARE_VERSION: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60002);
/// The current firmware version installed on the main processor.
pub const FIRMWARE_VERSION: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60004);
/// The bootloader version installed on the main processor.
pub const BOOTLOADER_VERSION: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60006);
/// The current firmware version of the WiFi module, if available.
pub const WIFI_VERSION: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60008);
/// Bitmask indicating installed hardware options. bit0: High Resolution ADC, bit1: WiFi, bit2: RTC, bit3: microSD.
pub const HARDWARE_INSTALLED: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60010);
/// Bitmask register that indicates why a device was last re-set. bit0: Power-on reset, bit1: Brown-out, bit2: Wake from idle, bit3: Wake from sleep, bit4: watchdog timer time-out, bit5: reserved, bit6: Software reset, bit7: external reset (MCLR), bit8: Voltage regulator standby, bit9: Configuration mismatch.
pub const DEVICE_RESET_DBG_REG: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60014);
/// The MAC address of the wired Ethernet module.
pub const LJMCOMMON_VERSION: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60018);
/// The MAC address of the wired Ethernet module.
pub const ETHERNET_MAC: LabjackTag<u64, CanRead, CannotWrite> = LabjackTag::new(60020);
/// The MAC address of the WiFi module.
pub const WIFI_MAC: LabjackTag<u64, CanRead, CannotWrite> = LabjackTag::new(60024);
/// The serial number of the device.
pub const SERIAL_NUMBER: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60028);
/// T8 Only. Returns the power supply voltage.
pub const INTERNAL_VS_VOLTS: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60030);
/// T8 Only. Returns the power supply current draw.
pub const INTERNAL_IS_AMPS: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60032);
/// Returns the estimated ambient air temperature just outside of the device in its red plastic enclosure.  This register is equal to TEMPERATURE_DEVICE_K - 4.3.  If Ethernet and/or WiFi is enabled, subtract an extra 0.6 for each.
pub const TEMPERATURE_AIR_K: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60050);
/// Takes a reading from the internal temperature sensor using range=+/-10V and resolution=8, and applies the formula Volts*-92.6+467.6 to return kelvins.
pub const TEMPERATURE_DEVICE_K: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(60052);
/// T8 Only. Returns the status of the calibration data saved in flash. 1 = calibration looks good. 0 = calibration invalid.
pub const CALIBRATION_CONSTANTS_STATUS: LabjackTag<f32, CanRead, CannotWrite> =
    LabjackTag::new(60091);
/// T8 Only. Sets and reads the source of the current set of calibration constants. 0 = no calibration. 1 = binary calibration, 2 = Calibration set from Flash, 3 = Nominal Calibration. When setting to 2, the constants will be checked and if invalid 1 will be used instead.
pub const CALIBRATION_SOURCE: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(60092);
/// Reads return the current device name. Writes update the default and current device name. A reboot is necessary to update the name reported by NBNS. Up to 49 characters, cannot contain periods.
pub const DEVICE_NAME_DEFAULT: LabjackTag<Bytes, CanRead, CanWrite> = LabjackTag::new(60500);
/// Write any value to this register to change the current working directory (CWD). Must first designate which directory to open by writing to FILE_IO_PATH_WRITE_LEN_BYTES then FILE_IO_PATH_WRITE.
pub const FILE_IO_DIR_CHANGE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60600);
/// Write any value to this register to load the current working directory into FILE_IO_PATH_READ, and its length into FILE_IO_PATH_READ_LEN_BYTES. Can be used to identify current position in a file tree.
pub const FILE_IO_DIR_CURRENT: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60601);
/// Unimplemented.
pub const FILE_IO_DIR_MAKE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60602);
/// Unimplemented.
pub const FILE_IO_DIR_REMOVE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60603);
/// Write any value to this register to initiate iteration through files and directories in the CWD. Typical sequence: FILE_IO_DIR_FIRST(W), then loop through: [FILE_IO_NAME_READ_LEN(R), FILE_IO_NAME_READ(R), FILE_IO_ATTRIBUTES(R), FILE_IO_SIZE_BYTES(R), FILE_IO_DIR_NEXT(W)] ..until any of the following errors: FILE_IO_END_OF_CWD (2966), FILE_IO_INVALID_OBJECT (2809), or FILE_IO_NOT_FOUND (2960).
pub const FILE_IO_DIR_FIRST: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60610);
/// Write any value to this register to continue iteration through files and directories in the CWD.
pub const FILE_IO_DIR_NEXT: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60611);
/// Write any value to this register to open a file. Must first designate which file to open by writing to FILE_IO_PATH_WRITE_LEN_BYTES then FILE_IO_PATH_WRITE.
pub const FILE_IO_OPEN: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60620);
/// Write any value to this register to close the open file.
pub const FILE_IO_CLOSE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60621);
/// Write any value to this register to delete the active file. Must first designate which file to delete by writing to FILE_IO_PATH_WRITE_LEN_BYTES then FILE_IO_PATH_WRITE.
pub const FILE_IO_DELETE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60622);
/// Used to differentiate files from directories/folders. Bitmask: Bit0: Reserved, Bit1: Reserved, Bit2: Reserved, Bit3: Reserved, Bit4: 1=Directory, Bit5: 1=File.
pub const FILE_IO_ATTRIBUTES: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(60623);
/// The size of the file in bytes. Directories have 0 size.
pub const FILE_IO_SIZE_BYTES: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60628);
/// The size of each sector in the SD card in bytes. In Windows this is called the Allocation Size.
pub const FILE_IO_DISK_SECTOR_SIZE_BYTES: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60630);
/// The number of sectors in each cluster. Captured on read of FILE_IO_DISK_SECTOR_SIZE_BYTES.
pub const FILE_IO_DISK_SECTORS_PER_CLUSTER: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60632);
/// The total number of clusters in the SD card. Captured on read of FILE_IO_DISK_SECTOR_SIZE_BYTES.
pub const FILE_IO_DISK_TOTAL_CLUSTERS: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60634);
/// Free (available) clusters in the SD card. Used to determine free space. Captured on read of FILE_IO_DISK_SECTOR_SIZE_BYTES.
pub const FILE_IO_DISK_FREE_CLUSTERS: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60636);
/// Used to determine the format of the SD card. 0=None or Unknown, 1=FAT12, 2=FAT16(Windows FAT), 3=FAT32
pub const FILE_IO_DISK_FORMAT_INDEX: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60638);
/// Write the length (in bytes) of the file path or directory to access.
pub const FILE_IO_PATH_WRITE_LEN_BYTES: LabjackTag<u32, CannotRead, CanWrite> =
    LabjackTag::new(60640);
/// Read the length (in bytes) of the next file path or directory to access.
pub const FILE_IO_PATH_READ_LEN_BYTES: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60642);
/// Write the desired file path. Must first write the length of the file path string (in bytes) to FILE_IO_PATH_WRITE_LEN_BYTES. File paths should be null terminated. This register is a buffer.
pub const FILE_IO_PATH_WRITE: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(60650);
/// Read the next file path in the CWD. Length of the string (in bytes) determined by FILE_IO_PATH_READ_LEN_BYTES. File paths will be null terminated. This register is a buffer. Underrun behavior - fill with zeros.
pub const FILE_IO_PATH_READ: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(60652);
/// Unimplemented. This register is a buffer.
pub const FILE_IO_WRITE: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(60654);
/// Read the contents of a file. Must first write to FILE_IO_OPEN. Size of the file (in bytes) determined by FILE_IO_SIZE_BYTES. This register is a buffer. Underrun behavior - throws an error.
pub const FILE_IO_READ: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(60656);
/// Read the current time in seconds since Jan, 1970, aka Epoch or Unix time. This value is calculated from the 80 MHz crystal, not the RTC 32 kHz crystal. Non-pro devices do not have a real time clock, so the reported time is either seconds since device startup or the time reported by the network time protocol over Ethernet. Pro devices have a real time clock that will be used to initialize this register at startup, the time can then be updated by NTP.
pub const RTC_TIME_S: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(61500);
/// A 10 kHz counter synchronized to RTC_TIME_S. This register can be appended to RTC_TIME_S as the decimal portion to get 0.1 ms resolution.
pub const SYSTEM_COUNTER_10KHZ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(61502);
/// Write a new timestamp to the RTC in seconds since Jan, 1970, aka Epoch or Unix timestamp.
pub const RTC_SET_TIME_S: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61504);
/// Write any value to instruct the T7 to update its clock from a SNTP server. Requires that SNTP_UPDATE_INTERVAL is non-zero.
pub const RTC_SET_TIME_SNTP: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61506);
/// Read six consecutive addresses of type UINT16, starting with this address. The result will be in year, month, day, hour, minute, second calendar format. i.e. [2014, 10, 21, 18, 55, 35]
pub const RTC_TIME_CALENDAR: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(61510);
/// Internal 32-bit system timer running at 1/2 core speed.
pub const CORE_TIMER: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(61520);
/// Internal 32-bit system timer running at 20Hz.
pub const SYSTEM_TIMER_20HZ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(61522);
/// This has been deprecated in favor of DAC1_FREQUENCY_OUT_ENABLE. 0 = off, 1 = output 10 Hz signal on SPC. Note that stream uses SPC for a timing output, so if you use this while streaming you will get a lot more counts.
pub const SPC_FREQUENCY_OUT_ENABLE_DEPRECATED: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(61530);
/// 0 = off, 1 = output 10 Hz signal on DAC1. The signal will be a square wave with peaks of 0 and 3.3V. Note that writing to DAC1 or enabling a stream out which is targeting DAC1 will disable this feature.
pub const DAC1_FREQUENCY_OUT_ENABLE: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61532);
/// Delays for x microseconds. Range is 0-100000. This operation is Blocking. While a blocking function is running no other registers can be read / written.
pub const WAIT_US_BLOCKING: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61590);
/// Write a 1 to enable the watchdog or a 0 to disable.  The watchdog must be disabled before writing any of the other watchdog registers (except for WATCHDOG_STRICT_CLEAR).
pub const WATCHDOG_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61600);
/// Configure Watchdog behavior on timeout. 1 = IO_CONFIG_SET_CURRENT_TO_FACTORY, 2 = IO_CONFIG_SET_CURRENT_TO_DEFAULT.
pub const WATCHDOG_ADVANCED_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61602);
/// When the device receives any communication over USB/Ethernet/WiFi, the watchdog timer is cleared.  If the watchdog timer is not cleared within the timeout period, the enabled actions will be done.
pub const WATCHDOG_TIMEOUT_S_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61604);
/// This specifies the initial timeout period at device bootup.  This is used until the first time the watchdog is cleared or timeout ... after that the normal timeout is used.
pub const WATCHDOG_STARTUP_DELAY_S_DEFAULT: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(61606);
/// Set to 1 to enable strict mode.
pub const WATCHDOG_STRICT_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(61610);
/// When set to strict mode, this is the value that must be written to the clear register.
pub const WATCHDOG_STRICT_KEY_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61612);
/// When running in strict mode, writing the key to this register is the only way to clear the watchdog. Writing to this register while not using strict mode will clear the watchdog.
pub const WATCHDOG_STRICT_CLEAR: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61614);
/// Timeout action:  Set to 1 to enable device-reset on watchdog timeout.
pub const WATCHDOG_RESET_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(61620);
/// Timeout action:  Set to 1 to enable DIO update on watchdog timeout.
pub const WATCHDOG_DIO_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61630);
/// The state high/low of the digital I/O after a Watchdog timeout. See DIO_STATE
pub const WATCHDOG_DIO_STATE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61632);
/// The direction input/output of the digital I/O after a Watchdog timeout. See DIO_DIRECTION
pub const WATCHDOG_DIO_DIRECTION_DEFAULT: LabjackTag<u32, CanRead, CanWrite> =
    LabjackTag::new(61634);
/// This register can be used to prevent the watchdog from changing specific IO. See DIO_INHIBIT for more information.
pub const WATCHDOG_DIO_INHIBIT_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61636);
/// Timeout action:  Set to 1 to enable DAC0 update on watchdog timeout.
pub const WATCHDOG_DAC0_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61640);
/// The voltage of DAC0 after a Watchdog timeout.
pub const WATCHDOG_DAC0_DEFAULT: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(61642);
/// Timeout action:  Set to 1 to enable DAC1 update on watchdog timeout.
pub const WATCHDOG_DAC1_ENABLE_DEFAULT: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61650);
/// The voltage of DAC1 after a Watchdog timeout.
pub const WATCHDOG_DAC1_DEFAULT: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(61652);
/// Sets the region of internal flash to which access is allowed.
pub const INTERNAL_FLASH_KEY: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61800);
/// The address in internal flash that reads will start from.
pub const INTERNAL_FLASH_READ_POINTER: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61810);
/// Data read from internal flash. Read size must be an even number of registers.
pub const INTERNAL_FLASH_READ: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(61812);
/// Erases a 4k section of internal flash starting at the specified address. This register is a buffer.
pub const INTERNAL_FLASH_ERASE: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61820);
/// Address in internal flash where writes will begin.
pub const INTERNAL_FLASH_WRITE_POINTER: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(61830);
/// Data written here will be written to internal flash. Write size must be an even number of registers. This register is a buffer.
pub const INTERNAL_FLASH_WRITE: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61832);
/// Calculates the checksum of a 4096 byte flash page.
pub const INTERNAL_FLASH_CALCULATE_CRC: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(61842);
/// Write a 1 to set current values to factory configuration.  The factory values are retrieved from flash and written to the current configuration registers. Systems affected: AIN, DIO, DAC, AIN_EF, DIO_EF.
pub const IO_CONFIG_SET_CURRENT_TO_FACTORY: LabjackTag<u16, CannotRead, CanWrite> =
    LabjackTag::new(61990);
/// Write a 1 to set current values to default configuration.  The default values are retrieved from flash and written to the current configuration registers, thus this behaves similar to reboot/power-up. Systems affected: AIN, DIO, DAC, AIN_EF, DIO_EF.
pub const IO_CONFIG_SET_CURRENT_TO_DEFAULT: LabjackTag<u16, CannotRead, CanWrite> =
    LabjackTag::new(61991);
/// Request a firmware update.  Pass a value of 150 (5mS) so that the Digit has time to de-enumerate. Only used by mfr.
pub const DGT_REQ_FW_UPDATE: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61996);
/// Issues a device reboot. Must write 0x4C4Axxxx, where xxxx is number of 50ms ticks to wait before reboot.  To reboot immediately write 0x4C4A0000 (d1279918080).
pub const SYSTEM_REBOOT: LabjackTag<u32, CannotRead, CanWrite> = LabjackTag::new(61998);
