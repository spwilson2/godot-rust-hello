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

static GDExample *second = NULL;

static std::mutex mtx;
static std::vector<Vector2> instantiate_queue;
static uint32_t count;

extern "C" { 
    void hello_world();
    void done_world();

    void gds_instantiate(uint32_t x, uint32_t y) {
        auto vec = Vector2(x,y);
        //mtx.lock();
        instantiate_queue.push_back(vec);
        //mtx.unlock();
        //auto new_node = (GDExample*) second->duplicate();
        //auto trans = Transform2D(0, vec);
        //new_node->set_transform(trans);
        //return (uintptr_t)new_node;
    }
    void gds_move(uintptr_t id, uint32_t x, uint32_t y) {
        auto node = (GDExample*)id;
        auto vec = Vector2(x,y);
        auto trans = Transform2D(0, vec);
        node->set_transform(trans);
    }
}


void GDExample::_process(double delta) {
    if (enabled) {
        return;
    }

    if (!second ) {
        second = (GDExample*) this->duplicate();
        auto vec = Vector2(20,200);
        auto trans = Transform2D(0, vec);
        second->set_transform(trans);
        this->add_sibling(second);
        // Spawn the rust thread.
        hello_world();
    }

    //mtx.lock();
    if (instantiate_queue.size() == 10000) {
    while (instantiate_queue.size() > 0) {
        Vector2 vec = instantiate_queue.back();
        instantiate_queue.pop_back();
        auto new_node = (GDExample*) this->duplicate();
        auto trans = Transform2D(0, vec);
        new_node->set_transform(trans);
        this->add_sibling(new_node);
        count++;
        new_node->set_z_index(count% 4096);
    }
    done_world();
}
    //mtx.unlock();

    //time_passed += delta;
    //Vector2 new_position = Vector2(10.0 + (10.0 * sin(time_passed * 20.0)), 10.0 + (10.0 * cos(time_passed * 10.5)));
    //set_position(new_position);
}