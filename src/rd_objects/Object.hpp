#pragma once

#include <array>







namespace RuntDealeObjects {





struct Object {
	Object(
		std::array<int, 2> coords,
		std::array<int, 2> bounds
	) {
		this->coords = coords;
		this->bounds = bounds;
	}
	Object(
		int coords[2],
		int bounds[2],
		unsigned int layer
	):Object{coords, bounds} {
		this->layer = layer;
	}
	
	Renderable* renderer = nullptr;
	std::array<int, 2> bounds;
	std::array<int, 2> coords;
	unsigned int layer = 1;
	bool solid = false;
};





}