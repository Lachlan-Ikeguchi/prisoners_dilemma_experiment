-- name: CYNIC
-- expects the worst

function decide(round, history)
    if round == 1 then return "defect" end
    return "defect"
end
