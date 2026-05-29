-- name: CON ARTIST
-- tricks opponent into cooperating

function decide(round, history)
    if round == 1 then return "cooperate" end
    if round == 2 then return "cooperate" end
    if round == 3 then return "defect" end
    
    return "defect"
end
