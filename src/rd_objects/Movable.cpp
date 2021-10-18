#pragma once

#include "Movable.hpp"
#include <array>







namespace RuntDealeObjects {





std::array<int 2> Movable::moveBy(
	int x, int y,
	bool factorInSpeed
) {
	if(factorInSpeed) {
		x *= this->speed;
		y *= this->speed;
	}
	this->coords[0] += x;
	this->coords[1] += y;
	
	return this->coords;
}
void Movable::moveTo(int x, int y) {
	this->coords[0] = x;
	this->coords[1] = y;
}





}