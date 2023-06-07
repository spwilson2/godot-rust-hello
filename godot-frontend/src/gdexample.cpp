#include "gdexample.h"
#include <godot_cpp/core/class_db.hpp>

using namespace godot;

void GDExample::_bind_methods() {
        ClassDB::bind_method(D_METHOD("get_enabled"), &GDExample::get_enabled);
        ClassDB::bind_method(D_METHOD("set_enabled", "value"), &GDExample::set_enabled);
        ClassDB::add_property("GDExample", PropertyInfo(Variant::FLOAT, "enabled"), "set_enabled", "get_enabled");
}

GDExample::GDExample() {
}

GDExample::~GDExample() {
}

extern "C" { 
    void hello_world();
}

void GDExample::_process(double delta) {
    if (!enabled) {
        hello_world();
        return;
    }
    time_passed += delta;

    Vector2 new_position = Vector2(10.0 + (10.0 * sin(time_passed * 20.0)), 10.0 + (10.0 * cos(time_passed * 10.5)));

    set_position(new_position);
}