import sqlite3
import json

best_results = {}
score_per_hash = {}

with sqlite3.connect("my_database.db") as conn:
    cursor = conn.cursor()
    cursor.execute("SELECT hash, actions, score FROM my_objects WHERE name LIKE 'Standard%'")
    rows = cursor.fetchall()
    for hash, action, score in rows:
        if score > score_per_hash.get(hash, 0):
            score_per_hash[hash] = score
            best_results[hash] = action

print(json.dumps(best_results, indent=4))
print("total score: ", 2 * sum(score_per_hash.values()))
