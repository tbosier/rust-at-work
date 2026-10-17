"""Build kettle_creek.db, the practice database for "Rust at work".

Kettle Creek Supply is a made-up industrial parts distributor. Everything in
the database is synthetic. The script is deterministic: the same seed gives
the same file, byte for byte in content, every time.

    python3 make_db.py            # writes kettle_creek.db next to this file

Money is generated in integer cents and stored as REAL dollars, the way many
real systems store it. That is deliberate: the learner has to deal with it.
"""

import datetime as dt
import random
import sqlite3
import sys
from pathlib import Path

SEED = 20261005
AS_OF = dt.date(2026, 10, 5)  # "today" in the simulation
FIRST_DAY = dt.date(2025, 1, 6)
LAST_ORDER_DAY = dt.date(2026, 10, 2)

SUPPLIERS = [
    # name, city, state, terms, lead_days, late_prob, weight, prefix
    ("Quarrow Bearing Co.", "Springfield", "MO", 30, 9, 0.10, 9, "QRB"),
    ("Vellmont Fasteners", "Dayton", "OH", 30, 6, 0.06, 14, "VMF"),
    ("Ostrand Hydraulics", "Wichita", "KS", 45, 18, 0.22, 7, "OSH"),
    ("Brannoch Electric Supply", "Tacoma", "WA", 30, 12, 0.12, 8, "BES"),
    ("Lisset Abrasives", "Erie", "PA", 30, 8, 0.08, 5, "LSA"),
    ("Corvane Ridge Safety", "Duluth", "MN", 15, 7, 0.05, 6, "CRV"),
    ("Saltreu Pneumatics", "Tucson", "AZ", 45, 21, 0.30, 4, "SRP"),
    ("Tamsk Ridge Seals & Gaskets", "Roanoke", "VA", 30, 10, 0.09, 5, "TRS"),
    ("Orlin Prairie Motors", "Fargo", "ND", 60, 28, 0.35, 3, "OPM"),
    ("Wendmoor Chain & Sprocket", "Knoxville", "TN", 30, 11, 0.11, 4, "WCS"),
    ("Pellish Valve Works", "Baton Rouge", "LA", 45, 16, 0.18, 5, "PVW"),
    ("Grenlow Hill Tooling", "Manchester", "NH", 30, 9, 0.07, 3, "GRT"),
    ("Mirra Stone Lubricants", "Durango", "CO", 30, 7, 0.06, 4, "MSL"),
    ("Ebberly Wire & Cable", "Lancaster", "PA", 30, 13, 0.14, 6, "EWC"),
    ("Kestrin Filtration", "Boise", "ID", 30, 10, 0.10, 3, "KTF"),
    ("Old Varrow Hardware", "Lowell", "MA", 15, 5, 0.04, 7, "OVH"),
    ("Sandovar Castings", "Lincoln", "NE", 60, 30, 0.40, 2, "SDC"),
    ("Pinnerly Packaging", "Bangor", "ME", 30, 6, 0.05, 3, "PNP"),
    ("Westrell Gear", "Rockford", "IL", 45, 19, 0.20, 4, "WRG"),
    ("Coravel Marine Hardware", "Mobile", "AL", 30, 14, 0.16, 2, "CMH"),
    ("Daskin Welding Supply", "Reno", "NV", 30, 8, 0.09, 4, "DWS"),
    ("Norrowfield Conveyor Parts", "Missoula", "MT", 45, 24, 0.28, 2, "NCP"),
    ("Jessamy Instruments", "Bend", "OR", 30, 15, 0.13, 2, "JSI"),
    ("Selwick Mere Plastics", "Saginaw", "MI", 30, 9, 0.08, 3, "SWP"),
]
INACTIVE = {"Coravel Marine Hardware", "Norrowfield Conveyor Parts", "Jessamy Instruments"}

CATEGORIES = {
    "Fasteners": (0.04, 2.50, 100, 2000, ["Hex bolt", "Lock washer", "Nylon insert nut", "Socket cap screw", "Carriage bolt", "Self-tapping screw"]),
    "Bearings": (4.00, 85.00, 4, 60, ["Deep groove ball bearing", "Tapered roller bearing", "Pillow block", "Needle bearing", "Flange bearing"]),
    "Hydraulics": (12.00, 340.00, 1, 20, ["Hydraulic hose assembly", "Quick coupler", "Gear pump", "Cylinder seal kit", "Pressure gauge"]),
    "Electrical": (0.80, 120.00, 5, 200, ["Contactor", "Terminal block", "Cable gland", "Proximity sensor", "Fuse holder"]),
    "Abrasives": (0.60, 18.00, 20, 400, ["Flap disc", "Cutoff wheel", "Sanding belt", "Wire brush", "Grinding wheel"]),
    "Safety": (0.90, 45.00, 10, 300, ["Nitrile gloves (box)", "Safety glasses", "Ear plugs (box)", "Hi-vis vest", "Face shield"]),
    "Power transmission": (3.00, 160.00, 2, 50, ["V-belt", "Roller chain", "Sprocket", "Shaft coupling", "Timing pulley"]),
    "Seals": (0.30, 25.00, 20, 500, ["O-ring", "Oil seal", "Gasket sheet", "Lip seal", "Backup ring"]),
}
SUPPLIER_CATEGORIES = {
    "QRB": ["Bearings", "Power transmission"], "VMF": ["Fasteners"], "OSH": ["Hydraulics", "Seals"],
    "BES": ["Electrical"], "LSA": ["Abrasives"], "CRV": ["Safety"], "SRP": ["Hydraulics"],
    "TRS": ["Seals"], "OPM": ["Electrical", "Power transmission"], "WCS": ["Power transmission"],
    "PVW": ["Hydraulics"], "GRT": ["Abrasives", "Fasteners"], "MSL": ["Seals"],
    "EWC": ["Electrical"], "KTF": ["Hydraulics", "Seals"], "OVH": ["Fasteners", "Safety"],
    "SDC": ["Power transmission"], "PNP": ["Safety"], "WRG": ["Power transmission", "Bearings"],
    "CMH": ["Fasteners"], "DWS": ["Abrasives", "Safety"], "NCP": ["Power transmission", "Bearings"],
    "JSI": ["Electrical"], "SWP": ["Seals", "Safety"],
}
BUYERS_MAIN = ["A. Okafor", "J. Lindqvist", "M. Reyes", "S. Tran"]
BUYER_MILLBROOK = "T. Haskins"


def business_days(start, end):
    d = start
    while d <= end:
        if d.weekday() < 5:
            yield d
        d += dt.timedelta(days=1)


def shift_business(d, n):
    """Move n business days forward from d."""
    while n > 0:
        d += dt.timedelta(days=1)
        if d.weekday() < 5:
            n -= 1
    return d


def money(cents):
    return round(cents / 100, 2)


def build(path):
    rng = random.Random(SEED)
    if path.exists():
        path.unlink()
    db = sqlite3.connect(path)
    db.executescript(
        """
        CREATE TABLE suppliers (
            supplier_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            city TEXT NOT NULL,
            state TEXT NOT NULL,
            payment_terms_days INTEGER NOT NULL,
            active INTEGER NOT NULL
        );
        CREATE TABLE products (
            sku TEXT PRIMARY KEY,
            description TEXT NOT NULL,
            category TEXT NOT NULL,
            supplier_id INTEGER NOT NULL,
            list_price REAL NOT NULL
        );
        CREATE TABLE purchase_orders (
            po_id INTEGER PRIMARY KEY,
            supplier_id INTEGER NOT NULL,
            branch TEXT NOT NULL,
            buyer TEXT NOT NULL,
            order_date TEXT NOT NULL,
            expected_date TEXT NOT NULL,
            received_date TEXT,
            status TEXT NOT NULL
        );
        CREATE TABLE po_lines (
            po_id INTEGER NOT NULL,
            line_no INTEGER NOT NULL,
            sku TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price REAL NOT NULL,
            PRIMARY KEY (po_id, line_no)
        );
        CREATE TABLE invoices (
            invoice_id INTEGER PRIMARY KEY,
            invoice_number TEXT NOT NULL,
            supplier_id INTEGER NOT NULL,
            po_id INTEGER NOT NULL,
            invoice_date TEXT NOT NULL,
            amount REAL NOT NULL
        );
        CREATE TABLE branch_invoices (
            ref TEXT,
            vendor TEXT,
            po_ref TEXT,
            inv_date TEXT,
            amount TEXT,
            entered_by TEXT
        );
        """
    )

    # Suppliers
    sup_rows = []
    for i, (name, city, state, terms, lead, late, weight, prefix) in enumerate(SUPPLIERS, start=1):
        sup_rows.append((i, name, city, state, terms, 0 if name in INACTIVE else 1))
    db.executemany("INSERT INTO suppliers VALUES (?,?,?,?,?,?)", sup_rows)

    # Products: each supplier carries 5 to 9 SKUs from its categories.
    products = {}  # supplier_id -> list of (sku, category, list_price_cents)
    prod_rows = []
    sku_seen = set()
    for sid, (_, _, _, _, _, _, _, prefix) in enumerate(SUPPLIERS, start=1):
        products[sid] = []
        for _ in range(rng.randint(5, 9)):
            cat = rng.choice(SUPPLIER_CATEGORIES[prefix])
            lo, hi, _, _, names = CATEGORIES[cat]
            while True:
                sku = f"{prefix}-{rng.randint(1000, 9999)}"
                if sku not in sku_seen:
                    sku_seen.add(sku)
                    break
            base = rng.choice(names)
            size = rng.choice(["", " 1/4in", " 3/8in", " 1/2in", " M8", " M10", " 25mm", " 40mm", " heavy duty", " stainless"])
            price = int(round(rng.uniform(lo, hi) * 100))
            products[sid].append((sku, cat, price))
            prod_rows.append((sku, base + size, cat, sid, money(price)))
    db.executemany("INSERT INTO products VALUES (?,?,?,?,?)", prod_rows)

    weights = [s[6] for s in SUPPLIERS]
    po_rows, line_rows, inv_rows, branch_rows = [], [], [], []
    po_id = 104_000
    inv_id = 500_000
    inv_seq = {s[7]: rng.randint(10_000, 40_000) for s in SUPPLIERS}

    def next_invoice_number(prefix):
        inv_seq[prefix] += rng.randint(1, 9)
        return f"{prefix}-{inv_seq[prefix]}"

    millbrook_pos = []  # (po_id, sid, expected_cents, received_date)

    for day in business_days(FIRST_DAY, LAST_ORDER_DAY):
        for _ in range(rng.choice([3, 4, 4, 5, 5, 6])):
            po_id += rng.choice([1, 1, 1, 1, 2])  # occasional gaps, like a real sequence
            sid = rng.choices(range(1, len(SUPPLIERS) + 1), weights=weights)[0]
            name, _, _, _, lead, late_p, _, prefix = SUPPLIERS[sid - 1]
            if name in INACTIVE and day > dt.date(2026, 3, 31):
                sid = 2
                name, _, _, _, lead, late_p, _, prefix = SUPPLIERS[1]
            millbrook = day >= dt.date(2025, 6, 2) and rng.random() < 0.13
            branch = "MILLBROOK" if millbrook else "MAIN"
            buyer = BUYER_MILLBROOK if millbrook else rng.choice(BUYERS_MAIN)

            # Lines
            expected = 0
            n_lines = rng.choices([1, 2, 3, 4, 5, 6], weights=[30, 25, 18, 12, 9, 6])[0]
            chosen = rng.sample(products[sid], min(n_lines, len(products[sid])))
            for ln, (sku, cat, list_cents) in enumerate(chosen, start=1):
                _, _, qlo, qhi, _ = CATEGORIES[cat]
                qty = rng.randint(qlo, qhi)
                if qty > 50:
                    qty = qty // 10 * 10
                price = max(1, int(round(list_cents * rng.uniform(0.92, 1.06))))
                expected += qty * price
                line_rows.append((po_id, ln, sku, qty, money(price)))

            lead_days = max(2, int(rng.gauss(lead, lead * 0.2)))
            expected_date = shift_business(day, lead_days)
            delay = 0
            if rng.random() < late_p:
                delay = rng.choice([1, 2, 3, 4, 5, 7, 10, 15])
            received = shift_business(expected_date, delay) if delay else expected_date
            if rng.random() < 0.15:
                received = shift_business(day, max(1, lead_days - rng.randint(1, 3)))

            if rng.random() < 0.03:
                status, received_s = "cancelled", None
            elif received >= AS_OF:
                status, received_s = "open", None
            elif rng.random() < 0.012 and day < dt.date(2026, 6, 1):
                # Received in the warehouse but nobody updated the PO: a stale open PO.
                status, received_s = "open", None
            else:
                status, received_s = "received", received.isoformat()

            po_rows.append((po_id, sid, branch, buyer, day.isoformat(), expected_date.isoformat(), received_s, status))

            # Invoices for main-branch POs live in `invoices`.
            if status == "cancelled":
                if rng.random() < 0.06 and not millbrook:
                    inv_id += 1
                    inv_rows.append((inv_id, next_invoice_number(prefix), sid, po_id,
                                     shift_business(day, rng.randint(5, 20)).isoformat(), money(expected)))
                continue
            if status == "open" or received_s is None:
                continue
            if millbrook:
                millbrook_pos.append((po_id, sid, expected, received))
                continue

            inv_date = received + dt.timedelta(days=rng.randint(0, 28))
            if inv_date > AS_OF:
                continue  # not billed yet: normal
            r = rng.random()
            if r < 0.025:
                continue  # never billed
            if r < 0.065:
                # Split billing: two invoices that add up exactly.
                part = int(expected * rng.uniform(0.3, 0.7))
                second = min(AS_OF, inv_date + dt.timedelta(days=rng.randint(3, 20)))
                for amt, d in ((part, inv_date), (expected - part, second)):
                    inv_id += 1
                    inv_rows.append((inv_id, next_invoice_number(prefix), sid, po_id, d.isoformat(), money(amt)))
                continue
            amount = expected
            if r < 0.115:
                amount = expected + rng.choice([3500, 4850, 2250, 6000, int(expected * rng.uniform(0.02, 0.12))])
            elif r < 0.135:
                amount = expected - max(100, int(expected * rng.uniform(0.03, 0.25)))
            inv_id += 1
            number = next_invoice_number(prefix)
            inv_rows.append((inv_id, number, sid, po_id, inv_date.isoformat(), money(amount)))
            if rng.random() < 0.013:
                # The same invoice keyed in twice.
                inv_id += 1
                again = min(AS_OF, inv_date + dt.timedelta(days=rng.randint(0, 6)))
                inv_rows.append((inv_id, number, sid, po_id, again.isoformat(), money(amount)))

    # Orphans: invoice po_id that matches no PO (digits transposed when keyed).
    known = {row[0] for row in po_rows}
    orphans = 0
    for row in rng.sample(inv_rows, 40):
        if orphans == 4:
            break
        s = str(row[3])
        swapped = int(s[:-2] + s[-1] + s[-2])
        if swapped != row[3] and swapped not in known:
            inv_id += 1
            inv_rows.append((inv_id, next_invoice_number(SUPPLIERS[row[2] - 1][7]), row[2], swapped, row[4], row[5]))
            orphans += 1

    inv_rows.sort(key=lambda r: (r[4], r[0]))  # entry order follows invoice date
    db.executemany("INSERT INTO purchase_orders VALUES (?,?,?,?,?,?,?,?)", po_rows)
    db.executemany("INSERT INTO po_lines VALUES (?,?,?,?,?)", line_rows)
    db.executemany("INSERT INTO invoices VALUES (?,?,?,?,?,?)", inv_rows)

    # Millbrook's invoices arrive from the acquired branch's old system: all text, all messy.
    clerks = ["dkemp", "dkemp", "rvasquez", "rvasquez", "temp01"]
    ref_n = 0
    for po, sid, expected, received in millbrook_pos:
        inv_date = received + dt.timedelta(days=rng.randint(0, 28))
        if inv_date > AS_OF or rng.random() < 0.04:
            continue
        name = SUPPLIERS[sid - 1][0]
        amounts = [expected]
        r = rng.random()
        if r < 0.06:
            over = rng.choice([2500, 3900, int(expected * 0.05)])
            amounts = [expected + over, -over]  # billed high, then credited back
        elif r < 0.10:
            amounts = [expected + rng.choice([3500, 4850])]
        for amt in amounts:
            ref_n += 1
            ref = f"MB-{inv_date.year}-{ref_n:04d}"
            vendor = dirty_vendor(rng, name)
            po_ref = dirty_po(rng, po)
            date_s = dirty_date(rng, inv_date)
            amount_s = dirty_amount(rng, amt)
            row = (ref, vendor, po_ref, date_s, amount_s, rng.choice(clerks))
            branch_rows.append(row)
            if rng.random() < 0.012:
                branch_rows.append(row)  # pasted twice
            inv_date = min(AS_OF, inv_date + dt.timedelta(days=rng.randint(2, 12)))
    # Dates that look fine and aren't: 2025 was not a leap year.
    for i, bad in zip(rng.sample(range(len(branch_rows)), 2), ["02/29/2025", "2025-02-29"]):
        branch_rows[i] = branch_rows[i][:3] + (bad,) + branch_rows[i][4:]
    # Two vendors Kettle Creek never set up.
    for vendor in ["Hollisford Freight Brokers", "Zippett Courier"]:
        ref_n += 1
        branch_rows.append((f"MB-2026-{ref_n:04d}", vendor, "", "09/03/2026", "185.00", "temp01"))
    db.executemany("INSERT INTO branch_invoices VALUES (?,?,?,?,?,?)", branch_rows)
    db.commit()
    db.execute("VACUUM")
    db.close()


def dirty_vendor(rng, name):
    r = rng.random()
    if r < 0.60:
        return name
    if r < 0.75:
        return name.upper()
    if r < 0.85:
        return "  " + name + " "
    if r < 0.93:
        return name.rstrip(".").replace("&", "and") if "&" in name else name.rstrip(".") + "  "
    if r < 0.97:
        return name.lower()
    return name.split()[0] + " " + name.split()[1]  # truncated: can't be matched safely


def dirty_po(rng, po):
    r = rng.random()
    if r < 0.50:
        return str(po)
    if r < 0.80:
        return f"PO-{po}"
    if r < 0.92:
        return f"PO {po}"
    if r < 0.96:
        return f" {po} "
    if r < 0.985:
        return ""
    return "see email"


def dirty_date(rng, d):
    r = rng.random()
    if r < 0.50:
        return d.isoformat()
    if r < 0.93:
        return f"{d.month:02d}/{d.day:02d}/{d.year}"
    if r < 0.96:
        return f"{d.month}/{d.day}/{d.year}"
    if r < 0.98:
        return ""
    # A day/month mix-up that produces an impossible date.
    return f"{max(d.day, 13):02d}/{d.month:02d}/{d.year}"


def dirty_amount(rng, cents):
    neg = cents < 0
    c = abs(cents)
    dollars, rem = divmod(c, 100)
    plain = f"{dollars}.{rem:02d}"
    commas = f"{dollars:,}.{rem:02d}"
    if neg:
        return rng.choice([f"({commas})", f"-{plain}"])
    r = rng.random()
    if r < 0.40:
        return plain
    if r < 0.70:
        return f"${commas}"
    if r < 0.82:
        return plain.rstrip("0").rstrip(".") if rem % 10 == 0 else plain
    if r < 0.92:
        return f" {commas} "
    if r < 0.955:
        return ""
    if r < 0.975:
        return "TBD"
    return f"{dollars}.{rem:02d}5"  # three decimals: fractional cents


if __name__ == "__main__":
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).with_name("kettle_creek.db")
    build(out)
    print(f"wrote {out}")
