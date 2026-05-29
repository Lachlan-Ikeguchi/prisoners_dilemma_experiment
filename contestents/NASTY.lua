-- name: NASTY
-- starts with defect, then mirrors opponent

function decide(round, history)
    if round == 1 then
        return "defect"
    end
    
    return history[#history]
end
