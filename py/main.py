import csv
import datetime
import json
from pathlib import Path

from flask import Flask, request

app = Flask(__name__)


@app.route("/push/", methods=["POST"])
def push():
    if request.method == "POST":
        sensor_id = request.headers.get("Sensor")
        print(sensor_id)
        print(request.data)

        # fields for all sensors
        base_fields = [
            "sensor_id",
            "sensor_type",
            "timestamp",
        ]
        # fields only for the GPU sensor
        fields = [
            "gpu_temperature",
            "gpu_load",
            "gpu_memory",
        ]
        fn = Path("test.csv")
        new_file = fn.exists()
        with open(fn, "a") as fp:
            writer = csv.DictWriter(
                fp, delimiter=";", lineterminator="\n", fieldnames=base_fields + fields
            )
            if not new_file:
                writer.writeheader()
            row = {
                "sensor_id": sensor_id,
                "sensor_type": "FIXME",  # uses lookup in production code
                "timestamp": str(datetime.datetime.now().isoformat()),
            }
            data = json.loads(request.data.decode("utf8"))
            for field in fields:
                for element in data.get("sensordatavalues"):
                    if element.get("value_type") == field:
                        row[field] = element.get("value")
                        break
            writer.writerow(row)
    return "OK"


@app.route("/")
def index():
    return "nothing to see."
