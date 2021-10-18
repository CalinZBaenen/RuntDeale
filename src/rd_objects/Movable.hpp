#pragma once

#include "Object.hpp"
#include <array>







namespace RuntDealeObjects {





struct Movable:Object {
	Movable(
		std::array<int, 2> coords,
		std::array<int, 2> bounds
	):Object{coords, bounds} {}
	Movable(
		std::array<int, 2> coords,
		std::array<int, 2> bounds,
		unsigned int layer
	):Object{coords, bounds, layer} {}
	
	std::array<int, 2> moveBy(int x, int y, bool factorInSpeed=true);
	void moveTo(int x, int y);
	
	int speed = 1;
};





}