#ifndef GDEXAMPLE_H
#define GDEXAMPLE_H

#include <godot_cpp/classes/sprite2d.hpp>

namespace godot {

class GDExample : public Sprite2D {
    GDCLASS(GDExample, Sprite2D)

private:
    double time_passed;
    bool enabled;

protected:
    static void _bind_methods();

public:
    GDExample();
    ~GDExample();

    bool get_enabled() {
        return enabled;
    }
    void set_enabled(bool v) {
        enabled = v;
    }

    void _process(double delta) override;
};

}

#endif