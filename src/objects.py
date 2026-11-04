# src.objects

class Species:
    """
    Class định nghĩa loài cá. Với tên, tốc độ bơi, tốc độ sinh sản, 
        khả năng chuyển hóa dinh dưỡng.
    """
    def __init__(self, name: str, 
                 speed: int, reproduction: int, metabolism: float):
        """
        Args:
        name: tên loài cá
        speed: tốc độ bơi (ô/frame)
        reproduction: tốc độ sinh sản (sau n frame) 
            | với gender female, sau khi touch gender male
        metabolism: khả năng chuyển hóa thức ăn
        """
        self.name = name
        self.speed = speed
        self.reproduction = reproduction
        self.metabolism = metabolism

    def __repr__(self):
        return f"Species({self.name})"

    def __str__(self):
        return (f"{self.name}")


class Fish:
    """
    Đại diện cho một con cá cụ thể
    """
    _id_counter = 0

    def __init__(self):
        pass



    