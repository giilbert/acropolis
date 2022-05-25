#include "rendering/2d/SpriteBatch.h"

#include <glm/vec4.hpp>
#include <glm/gtx/string_cast.hpp>
#include <iostream>

using giz::SpriteBatch;

SpriteBatch::SpriteBatch()
{
    // generate indices
    int *indices = new int[INDICES_IN_BATCH];
    int j = 0;
    for (int i = 0; i < 2 * 6; i += 6, j += 4)
    {
        indices[i + 0] = (j + 0);
        indices[i + 1] = (j + 2);
        indices[i + 2] = (j + 1);
        indices[i + 3] = (j + 0);
        indices[i + 4] = (j + 3);
        indices[i + 5] = (j + 2);
    }

    glGenVertexArrays(1, &m_VaoId);
    std::cout << "vao id: " << m_VaoId << "\n";
    glBindVertexArray(m_VaoId);

    unsigned int vertexBuffer, indexBuffer;
    glCreateBuffers(1, &vertexBuffer);
    glCreateBuffers(1, &indexBuffer);

    glBindBuffer(GL_ARRAY_BUFFER, vertexBuffer);
    glBufferData(GL_ARRAY_BUFFER, VERTICES_IN_BATCH * sizeof(float) * 4, nullptr, GL_DYNAMIC_DRAW);

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, INDICES_IN_BATCH * sizeof(int), indices, GL_STATIC_DRAW);
    delete indices;

    glVertexAttribPointer(0, 4, GL_FLOAT, GL_FALSE, 4 * sizeof(float), (void *)0);
    glEnableVertexAttribArray(0);

    m_VertexBuffer = vertexBuffer;
}

void SpriteBatch::Draw()
{
    glm::vec4 vertices[VERTICES_IN_BATCH];

    unsigned int i = 0;
    for (const auto &sprite : m_Sprites)
    {
        vertices[i] = sprite->m_Vertices[0];
        vertices[i + 1] = sprite->m_Vertices[1];
        vertices[i + 2] = sprite->m_Vertices[2];
        vertices[i + 3] = sprite->m_Vertices[3];
        i += 4;
    }

    glBindVertexArray(m_VaoId);
    glBindBuffer(GL_ARRAY_BUFFER, m_VertexBuffer);
    glBufferSubData(GL_ARRAY_BUFFER, 0, VERTICES_IN_BATCH, &vertices);
    glDrawElements(GL_TRIANGLES, INDICES_IN_BATCH, GL_UNSIGNED_INT, nullptr);
}

void SpriteBatch::AddSprite(component::Sprite *sprite)
{
    m_Sprites.push_back(sprite);
}