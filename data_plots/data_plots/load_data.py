from typing import List
from pathlib import Path
from dataclasses import dataclass


@dataclass
class Data:
    iteraction: int
    distance: float
    temperature: float


def load(fp: str) -> List[Data]:
    data_list = list()
    with Path(fp).open("r") as fd:
        lines = fd.readlines()
        for line in lines:
            raw_data = line.split(" ")
            iteraction = int(raw_data[0])
            dist = float(raw_data[1])
            temp = float(raw_data[2])
            if dist > 120_000:
                continue
            data = Data(iteraction, dist, temp)
            data_list.append(data)
    return data_list
