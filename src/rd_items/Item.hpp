#pragma once

#include "../rd_objects/Entity.hpp"
#include <string>







namespace RuntDealeItems {





struct Item {
	Item(std::string name) { this->name = name; }
	Item(std::string name, std::string description):Item{name} {
		this->description = description;
	}
	Item(
		std::string name, std::string description,
		std::string flavorText
	):Item{name, description} { this->flavorText = flavorText; }
	
	virtual void use() {}
	virtual void use(Entity e) {}
	
	std::string description = "";
	std::string flavorText = "";
	std::string name = "";
}





}