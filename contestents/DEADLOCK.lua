-- name: DEADLOCK
-- tries to force a stalemate

function decide(round, history)
    if round == 1 then return "defect" end
    return history[#history]
end
