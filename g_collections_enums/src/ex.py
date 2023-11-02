class Shot:
    def __init__(self, shot_type, distance=None):
        self.shot_type = shot_type
        self.distance = distance

    def points(self):
        if self.shot_type == "Bullseye":
            return 5
        elif self.shot_type == "Hit" and self.distance is not None:
            if self.distance < 3.0:
                return 2
            else:
                return 1
        else:
            return 0

# Thử nghiệm đoạn mã
shot1 = Shot("Bullseye")
print(f"Shot 1 points: {shot1.points()}")  # Kết quả: 5

shot2 = Shot("Hit", 2.5)
print(f"Shot 2 points: {shot2.points()}")  # Kết quả: 2

shot3 = Shot("Hit", 4.0)
print(f"Shot 3 points: {shot3.points()}")  # Kết quả: 1

shot4 = Shot("Miss")
print(f"Shot 4 points: {shot4.points()}")  # Kết quả: 0

