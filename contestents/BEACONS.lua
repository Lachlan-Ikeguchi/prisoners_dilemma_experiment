-- name: BEACONS
-- defects every 7 rounds

function decide(round, history)
    if round % 7 == 0 then return "defect" end
    return "cooperate"
end
