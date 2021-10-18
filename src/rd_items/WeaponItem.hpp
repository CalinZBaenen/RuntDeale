#pragma once

#include "../rd_objects/Entity.hpp"
#include <string>
#include "Item.hpp"







namespace RuntDealeItems {





struct WeaponItem:Item {
	WeaponItem(std::string name):Item{name} {}
	WeaponItem(
		std::string name, std::string description
	):Item{name, description} {}
	WeaponItem(
		std::string name, std::string description,
		std::string flavorText
	):Item{name, description, flavorText} {}
	WeaponItem(
		std::string name,
		int dmg
	):Item{name} { this->damage = dmg; }
	WeaponItem(
		std::string name, std::string description,
		int dmg
	):Item{name, description} { this->damage = dmg; }
	WeaponItem(
		std::string name, std::string description,
		std::string flavorText, int dmg
	):Item{name, description, flavorText} { this->damage = dmg; }
	
	void use(Entity attacker, Entity victim) {
		//
	}
	
	int damage = 0;
}





}