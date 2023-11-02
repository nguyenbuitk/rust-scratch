#include <iostream>
using namespace std;

enum class Shot {
    Bullseye,
    Hit,
    Miss
};

int points(Shot shot) {
    switch (shot) {
        case Shot::Bullseye:
            return 5;
        case Shot::Hit:
            return 2;
        case Shot::Miss:
            return 0;
    }
}

int main() {
    // Thêm mã để thử nghiệm enum và hàm points ở đây.
    // Ví dụ:
    Shot shot = Shot::Hit;
    int score = points(shot);
    cout << "Score: " << score << endl;

    return 0;
}

